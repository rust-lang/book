import tomlkit
import pandas as pd
import json
from pathlib import Path

def load_quizzes():
    QUIZ_DIR = Path('../../quizzes')
    quizzes = {}
    for quiz in QUIZ_DIR.iterdir():    
        with quiz.open() as f:
            quizzes[quiz.stem] = tomlkit.load(f)
    return quizzes

def load_log(name):
    with open(f'../server/{name}.log', 'r') as f:
        log = f.readlines()
    def load_line(line):
        obj = json.loads(line)
        payload = obj.pop('payload')
        return {**obj, **payload}
    return pd.DataFrame([load_line(line) for line in log])

