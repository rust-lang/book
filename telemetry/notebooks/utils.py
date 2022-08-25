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

