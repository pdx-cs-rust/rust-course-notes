#!/usr/bin/env python3

import argparse
import json
import re
import subprocess
from pathlib import Path


def sections(lines, level):
    marker = "#" * level + " "
    starts = [i for i, line in enumerate(lines)
              if line.startswith(marker)]
    result = []
    for n, start in enumerate(starts):
        end = starts[n + 1] if n + 1 < len(starts) else len(lines)
        result.append((lines[start][len(marker):].strip(),
                       lines[start + 1:end]))
    return result


def metadata(lines):
    result = {}
    for line in lines:
        match = re.fullmatch(r"- ([^:]+): (.*)", line)
        if match:
            result[match.group(1)] = match.group(2)
    return result


def render(markdown, pandoc, output):
    if not markdown.strip():
        return ""
    completed = subprocess.run(
        [pandoc, "--from=markdown", f"--to={output}",
         "--no-highlight"],
        input=markdown,
        text=True,
        check=True,
        capture_output=True,
    )
    return completed.stdout.strip()


def parse_answer(lines, pandoc):
    parts = sections(lines, 4)
    first_heading = next(
        (i for i, line in enumerate(lines) if line.startswith("#### ")),
        len(lines),
    )
    meta = metadata(lines[:first_heading])
    bodies = {name: "\n".join(body).strip()
              for name, body in parts}
    text = bodies["Text"]
    answer = {
        "answer_text": render(text, pandoc, "plain"),
        "answer_html": render(text, pandoc, "html5"),
        "answer_weight": int(meta["Weight"]),
    }
    if bodies.get("Feedback"):
        answer["answer_comments"] = render(
            bodies["Feedback"], pandoc, "html5"
        )
    return answer


def parse_question(heading, lines, pandoc):
    parts = sections(lines, 3)
    first_heading = next(
        (i for i, line in enumerate(lines) if line.startswith("### ")),
        len(lines),
    )
    meta = metadata(lines[:first_heading])
    part_map = {name: body for name, body in parts}
    prompt = "\n".join(part_map["Prompt"]).strip()
    number, _, name = heading.partition(":")
    type_map = {
        "multiple choice": "multiple_choice_question",
        "multiple answer": "multiple_answers_question",
        "true or false": "true_false_question",
    }
    answers = [parse_answer(body, pandoc)
               for part_name, body in parts
               if part_name.startswith("Answer ")]
    return {
        "number": int(number.removeprefix("Question ")),
        "question_name": name.strip() or "Question",
        "question_type": type_map[meta["Type"]],
        "points_possible": int(meta["Points"]),
        "question_text": render(prompt, pandoc, "html5"),
        "answers": answers,
    }


def parse_quiz(path, pandoc):
    lines = path.read_text().splitlines()
    if not lines or not lines[0].startswith("# "):
        raise ValueError("quiz must begin with a level-one title")
    top = sections(lines, 2)
    first_section = next(i for i, line in enumerate(lines)
                         if line.startswith("## "))
    preamble = "\n".join(lines[1:first_section]).strip()
    named = {name: body for name, body in top
             if not name.startswith("Question ")}
    settings = metadata(named["Settings"])
    instructions = "\n".join(named["Instructions"]).strip()
    description = "\n\n".join(
        item for item in (preamble, instructions) if item
    )
    questions = [parse_question(name, body, pandoc)
                 for name, body in top
                 if name.startswith("Question ")]
    questions.sort(key=lambda question: question["number"])
    return {
        "title": lines[0][2:].strip(),
        "settings": settings,
        "description": render(description, pandoc, "html5"),
        "questions": questions,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("quiz", type=Path)
    parser.add_argument("--pandoc", default=".tools/bin/pandoc")
    args = parser.parse_args()
    print(json.dumps(parse_quiz(args.quiz, args.pandoc), indent=2))


if __name__ == "__main__":
    main()
