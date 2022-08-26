import tomlkit
import pandas as pd
import json
from pathlib import Path
from datetime import datetime
import subprocess as sp
import bisect


class Quizzes:
    QUIZ_DIR = Path('../../quizzes')
    quizzes = {}

    def get(self, row):
        name = row.quizName
        if name not in self.quizzes:
            self.quizzes[name] = {
                'schemas': {},
                'dates': []
            }
        versions = self.quizzes[name]
        
        content_hash = row.quizHash
        commit_hash = row.commitHash.strip()
        if content_hash not in versions['schemas']:
            full_path = self.QUIZ_DIR / (name + '.toml')

            try:
                date_str = sp.check_output(['git', 'show', '-s', '--format=%ci', commit_hash]).decode('utf-8').strip()
                date = datetime.strptime(date_str, '%Y-%m-%d %H:%M:%S %z')            
                bisect.insort(versions['dates'], date)
                version = versions['dates'].index(date)

                schema_str = sp.check_output(['git', 'show', f'{commit_hash}:{full_path}']).decode('utf-8')
                schema = tomlkit.loads(schema_str)
                versions['schemas'][content_hash] = {'version': version, 'schema': schema}
            except Exception:
                print(f'Could not parse: {name} for {commit_hash}')

        return versions['schemas'][content_hash]


def load_log(name):
    with open(f'../server/{name}.log', 'r') as f:
        log = f.readlines()

    entries = []
    for line in log:
        try:
            obj = json.loads(line)
            payload = obj.pop('payload')
            entries.append({**obj, **payload})
        except Exception:
            print(f'Failed to load: {line}')

    return pd.DataFrame(entries)



def load_latest_answers():
    answers = load_log('answers')
    quizzes = Quizzes()

    # Load in all quiz data and get version metadata
    for _, row in answers.iterrows():
        quizzes.get(row)    

    # Convert hashes to version numbers
    answers['version'] = answers.apply(lambda row: quizzes.get(row)['version'], axis=1)

    # Convert UTC timestamp to datetime
    answers['timestamp'] = pd.to_datetime(answers['timestamp'], unit='ms')

    # Only keep the first attempt
    answers = answers[answers.attempt == 0]

    # Remove example data
    answers = answers[answers.quizName != 'example-quiz']

    # Only keep the latest complete answer for a given user/quiz pair
    get_latest = lambda group: group.iloc[group.timestamp.argmax()]
    did_complete_quiz = lambda row: len(row.answers) == len(quizzes.get(row)['schema']['questions'])
    groups = ['sessionId', 'quizName', 'quizHash']
    answers = answers \
        .loc[lambda df: df.apply(did_complete_quiz, axis=1)] \
        .groupby(groups) \
        .apply(get_latest) \
        .drop(columns=groups) \
        .reset_index()

    answers['frac_correct'] = answers.answers.map(lambda a: len([o for o in a if o['correct']]) / len(a))

    answers_flat = []
    for _, response in answers.iterrows():
        for i, ans in enumerate(response.answers):
            row = {**response, **ans, 'question': i}
            del row['answers']
            answers_flat.append(row)

    answers_flat = pd.DataFrame(answers_flat)

    return answers, answers_flat, quizzes
