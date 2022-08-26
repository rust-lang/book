import React, { useState } from "react";
import ReactDOM from "react-dom/client";
import DataGrid, {
  Column,
  SortColumn,
  headerRenderer as HeaderRenderer,
} from "react-data-grid";
import Slider from "rc-slider";

import "../styles.scss";
import "../index.html";
import _ from "lodash";

declare global {
  var QUESTION_SUMMARY: any;
  var QUIZ_SUMMARY: any;
}

type Comparator<T> = (a: T, b: T) => number;

interface BaseTypes {
  number: number;
  string: string;
}

let comparators: { [K in keyof BaseTypes]: Comparator<BaseTypes[K]> } = {
  number: (a, b) => a - b,
  string: (a, b) => a.localeCompare(b),
};

let useSortColumns = <T,>(rows: T[]) => {
  let [sortColumns, setSortColumns] = useState<readonly SortColumn[]>([]);
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

type FilterView<K> = React.FC<{ values: K[]; setFilter: (val: any) => void }>;

let filterViews: { [K in keyof BaseTypes]: FilterView<BaseTypes[K]> } = {
  number: ({ values, setFilter }) => {
    let min = _.min(values)!;
    let max = _.max(values)!;
    return (
      <Slider
        range
        min={min}
        max={max}
        step={(max - min) / values.length}
        marks={_.fromPairs(
          _.range(min, max, (max - min) / 5)
            .concat([max])
            .map((n) => [n, n.toFixed(max > 1 ? 0 : 1)])
        )}
        defaultValue={[min, max]}
        onChange={setFilter}
      />
    );
  },
  string: ({ setFilter }) => {
    return <input type="text" onChange={(e) => setFilter(e.target.value)} />;
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
  columns: Column<T>[]
): [T[], Column<T>[]] => {
  let [filters, setFilters] = useState<{ [k: string]: any }>({});
  let baseRow: any = rows[0];

  columns = columns.map((column) => {
    if (column.key in baseRow) {
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
      return keep;
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
}: {
  rows: T[];
  tableName: string;
  metricName: string;
  extraColumns?: Column<T>[];
}) {
  let columns: Column<T>[] = [
    { key: "quizName", name: "Quiz" },
    { key: "version", name: "Version" },
    ...(extraColumns || []),
    {
      key: "mean",
      name: metricName,
      formatter: ({ row: r }) => r.mean.toFixed(2),
    },
    {
      key: "ci",
      sortable: false,
      name: "Confidence Interval",
      formatter: ({ row: r }) =>
        r.lower ? `[${r.lower.toFixed(2)} - ${r.upper.toFixed(2)}]` : "n/a",
    },
    { key: "N", name: "N" },
  ];

  [rows, columns] = useFilterColumns(rows, columns);
  let sortProps = useSortColumns(rows);

  return (
    <div className="data-grid">
      <h2>{tableName}</h2>
      <DataGrid
        {...sortProps}
        headerRowHeight={70}
        defaultColumnOptions={{
          sortable: true,
        }}
        columns={columns}
      />
    </div>
  );
}

let App = () => {
  let questionSummary = JSON.parse(QUESTION_SUMMARY);
  let quizSummary = JSON.parse(QUIZ_SUMMARY);
  return (
    <div>
      <h1>Rust Book Experiment Dashboard</h1>
      <div className="grids">
        <Table
          tableName="Quiz Summary"
          rows={quizSummary}
          metricName="Avg quiz score"
        />
        <Table
          tableName="Question Summary"
          rows={questionSummary}
          metricName="Avg question score"
          extraColumns={[
            {
              key: "question",
              name: "Question",
            },
          ]}
        />
      </div>
    </div>
  );
};

let root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(<App />);
