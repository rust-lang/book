import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom/client";
import DataGrid, {
  Column,
  SortColumn,
  headerRenderer as HeaderRenderer,
} from "react-data-grid";
import { Range, createSliderWithTooltip } from "rc-slider";
import { action } from "mobx";
import { observer, useLocalObservable } from "mobx-react";
import { AnswerView, getQuestionMethods } from "@wcrichto/mdbook-quiz";
import hljs from "highlight.js";
import _ from "lodash";
import objectHash from "object-hash";
import axios from "axios";

import "../styles.scss";
import "../index.html";

declare global {
  var QUESTION_SUMMARY: string;
  var QUIZ_SUMMARY: string;
  var QUIZ_SCHEMAS: string;
  var hljs: any;
}

window.hljs = hljs;

type Comparator<T> = (a: T, b: T) => number;

interface BaseTypes {
  number: number;
  string: string;
}

let comparators: { [K in keyof BaseTypes]: Comparator<BaseTypes[K]> } = {
  number: (a, b) => a - b,
  string: (a, b) => a.localeCompare(b),
};

let useSortColumns = <T,>(rows: T[], def: SortColumn[] = []) => {
  let [sortColumns, setSortColumns] = useState<readonly SortColumn[]>(def);
  rows = [...(rows as any)].sort((ra, rb) => {
    for (const sort of sortColumns) {
      let a = ra[sort.columnKey],
        b = rb[sort.columnKey];

      if (!(typeof a in comparators)) {
        throw new Error(
          `Could not compare values in column: ${sort.columnKey}`
        );
      }

      let comparator = (comparators as any)[typeof a];
      return comparator(a, b) * (sort.direction === "ASC" ? 1 : -1);
    }
    return 0;
  });
  return { rows, sortColumns, onSortColumnsChange: setSortColumns };
};

type FilterView<K> = React.FC<{
  values: K[];
  defaultFilter?: any;
  setFilter: (val: any) => void;
}>;

let RangeTooltip = createSliderWithTooltip(Range);
let filterViews: { [K in keyof BaseTypes]: FilterView<BaseTypes[K]> } = {
  number: ({ values, defaultFilter, setFilter }) => {
    let min = _.min(values)!;
    let max = _.max(values)!;
    return (
      <RangeTooltip
        min={min}
        max={max}
        step={(max - min) / values.length}
        marks={_.fromPairs(
          _.range(min, max, (max - min) / 5)
            .concat([max])
            .map((n) => [n, n.toFixed(max > 1 ? 0 : 1)])
        )}
        defaultValue={defaultFilter || [min, max]}
        tipFormatter={(n) => n.toFixed(max > 1 ? 0 : 1)}
        onChange={setFilter}
      />
    );
  },
  string: ({ setFilter }) => {
    return (
      <input
        type="text"
        onClick={(e) => e.stopPropagation()}
        onFocus={(e) => e.stopPropagation()}
        onChange={(e) => setFilter(e.target.value)}
      />
    );
  },
};

let filterFuncs: {
  [K in keyof BaseTypes]: (filter: any, value: BaseTypes[K]) => boolean;
} = {
  number: ([lo, hi]: any, n: number) => lo <= n && n <= hi,
  string: (needle: string, haystack: string) => haystack.includes(needle),
};

let useFilterColumns = <T,>(
  rows: T[],
  columns: Column<T>[],
  defaultFilter: { [k: string]: any } = {}
): [T[], Column<T>[]] => {
  let [filters, setFilters] = useState<{ [k: string]: any }>(defaultFilter);
  let baseRow: any = rows[0];

  columns = columns.map((column) => {
    if (column.key in baseRow && column.sortable) {
      let kt = typeof baseRow[column.key];
      if (kt === "number" || kt === "string") {
        let F = filterViews[kt];
        let values = rows.map((r: any) => r[column.key]);
        return {
          ...column,
          headerCellClass: "filter-cell",
          headerRenderer: (props) => (
            <>
              <div>
                <HeaderRenderer {...props} />
              </div>
              <div>
                <F
                  values={values}
                  defaultFilter={filters[column.key]}
                  setFilter={(val: any) => {
                    setFilters({ ...filters, [column.key]: val });
                  }}
                />
              </div>
            </>
          ),
        };
      }
    }
    return column;
  });

  rows = rows.filter((r: any) => {
    for (let col of Object.keys(filters)) {
      let value = baseRow[col];
      let fn = filterFuncs[typeof value as "number" | "string"] as any;
      let keep = fn(filters[col], r[col]);
      if (!keep) return false;
    }
    return true;
  });

  return [rows, columns];
};

function Table<T extends { mean: number; lower: number; upper: number }>({
  rows,
  tableName,
  metricName,
  extraColumns,
  defaultSort,
  defaultFilter,
  state,
}: {
  rows: T[];
  tableName: string;
  metricName: string;
  extraColumns?: Column<T>[];
  defaultSort?: SortColumn[];
  defaultFilter?: { [k: string]: any };
  state: any;
}) {
  let columns: Column<T>[] = [
    { key: "quizName", name: "Quiz", sortable: true },
    { key: "version", name: "Version" },
    ...(extraColumns || []),
    {
      key: "mean",
      name: metricName,
      formatter: ({ row: r }) => r.mean.toFixed(2),
      sortable: true,
    },
    {
      key: "ci",
      sortable: false,
      name: "Confidence Interval",
      formatter: ({ row: r }) =>
        r.lower ? `[${r.lower.toFixed(2)} - ${r.upper.toFixed(2)}]` : "n/a",
    },
    { key: "N", name: "N", sortable: true },
  ];

  [rows, columns] = useFilterColumns(rows, columns, defaultFilter);
  let sortProps = useSortColumns(rows, defaultSort);

  return (
    <div className="data-grid">
      <h2>{tableName}</h2>
      <DataGrid
        {...sortProps}
        headerRowHeight={70}
        columns={columns}
        onRowClick={action((row: any) => {
          if ("question" in row) {
            state.selectedQuestion = row;
          }
        })}
      />
    </div>
  );
}

let QuestionInspector: React.FC<{ state: any; quizSchemas: any }> = observer(
  ({ state, quizSchemas }) => {
    if (!state.selectedQuestion) return null;
    let r = state.selectedQuestion;
    let schemas = quizSchemas[r.quizName];
    let hash = Object.keys(schemas).find(
      (k) => schemas[k].version == r.version
    )!;
    let schema = schemas[hash].schema;
    let question = schema.questions[r.question];

    let answers: { [hash: string]: { count: number; answer: any } } = {};
    for (let answer of r.answer) {
      let hash = objectHash(answer);
      if (!(hash in answers)) {
        answers[hash] = { count: 0, answer };
      }
      answers[hash].count += 1;
    }

    let answersSorted = _.chain(answers)
      .values()
      .sortBy((t) => -t.count)
      .value();

    let methods = getQuestionMethods(question.type);

    return (
      <div key={r.quizName + "/" + r.question}>
        <h2>
          Quiz {r.quizName} / Question {r.question + 1}
        </h2>
        <div className="qa-wrapper">
          <div>
            <h3>Question</h3>
            <div className="mdbook-quiz">
              <AnswerView
                index={r.question + 1}
                quizName={r.quizName}
                question={question}
                correct={true}
                userAnswer={question.answer}
                showCorrect={false}
              />
            </div>
          </div>
          <div>
            <h3>Answers</h3>
            {answersSorted.map((answer, i) => (
              <div className="wrong-answer" key={i}>
                <div>N = {answer.count}</div>
                <methods.AnswerView
                  answer={answer.answer}
                  baseline={question.answer}
                  prompt={question.prompt}
                />
              </div>
            ))}
          </div>
        </div>
      </div>
    );
  }
);

let App = () => {
  let state = useLocalObservable(() => ({
    selectedQuestion: null,
  }));
  let [data, setData] = useState<null | any>(() => null);

  useEffect(() => {
    (async () => {
      let files = [
        "data/question-summary.json",
        "data/quiz-summary.json",
        "data/quiz-schemas.json",
      ];
      let pairs = await Promise.all(
        files.map(async (url) => {
          let resp = await axios.get(url, {
            headers: {
              "Cache-Control": "no-cache",
              Pragma: "no-cache",
              Expires: "0",
            },
          });
          return [url, resp.data];
        })
      );
      setData(_.fromPairs(pairs));
    })();
  }, []);

  if (data == null) return <div>Loading data...</div>;

  return (
    <div>
      <h1>Rust Book Experiment Dashboard</h1>
      <div className="grids">
        <Table
          tableName="Quiz Summary"
          rows={data["data/quiz-summary.json"]}
          metricName="Avg quiz score"
          state={state}
        />
        <Table
          tableName="Question Summary"
          rows={data["data/question-summary.json"]}
          metricName="Avg question score"
          extraColumns={[
            {
              key: "question",
              name: "Question",
              formatter: ({ row }: { row: any }) => row.question + 1,
            },
          ]}
          defaultSort={[{ columnKey: "mean", direction: "ASC" }]}
          defaultFilter={{ N: [30, 10000] }}
          state={state}
        />
      </div>
      <QuestionInspector
        state={state}
        quizSchemas={data["data/quiz-schemas.json"]}
      />
    </div>
  );
};

let root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(<App />);
