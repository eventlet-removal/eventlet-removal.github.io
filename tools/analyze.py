import requests
import os
import argparse
import pathlib
import re
import yaml
from collections import defaultdict


def parse_beagle_output(output):
    projects = {}
    lines = output.splitlines()
    for line in lines:
        match = re.match(r"^(https?://[^\s]+) : (.+)$", line)
        if not match:
            continue
        url = match.group(1)
        line_content = match.group(2)
        project_name = url.split("/")[4]
        if project_name not in projects:
            projects[project_name] = []
        projects[project_name].append(f"{url} : {line_content}")
    return projects


def read_beagle_analysis(beagle_file):
    try:
        with open(beagle_file, "r") as file:
            content = file.read()
        return parse_beagle_output(content)
    except Exception as e:
        print(f"Error reading Beagle file '{beagle_file}':", e)
        return None


def read_project_teams(yaml_file):
    try:
        with open(yaml_file, "r") as file:
            project_teams = yaml.safe_load(file)
        team_to_projects = defaultdict(list)
        for team, details in project_teams.items():
            projects = details.get("deliverables", {})
            for project in projects.keys():
                team_to_projects[team].append(project)
        return team_to_projects
    except Exception as e:
        print(f"Error reading YAML file '{yaml_file}':", e)
        return None


def analyze_with_ollama(project_name, data, prompt, url, model, temperature=0.3):
    try:
        headers = {"Content-Type": "application/json"}
        payload = {
            "model": model,
            "prompt": f"{prompt}\n\nData for project {project_name}: "
            + "\n".join(data),
            "options": {
                "temperature": temperature
            },
            "stream": False,
        }
        response = requests.post(url, json=payload, headers=headers)
        if response.status_code != 200:
            print(f"Ollama analysis failed for {project_name}:", response.text)
            return None
        response_json = response.json()
        return response_json.get("response")
    except Exception as e:
        print(f"Error executing Ollama for {project_name}:", e)
        return None


def store_analysis_result(filename, analysis_result, output_dir):
    filepath = os.path.join(output_dir, filename)
    try:
        with open(filepath, "w") as file:
            file.write(analysis_result)
    except Exception as e:
        print(f"Error writing analysis result to '{filename}':", e)


def main():
    parser = argparse.ArgumentParser(
        description="Analysis of OpenStack projects using eventlet."
    )
    parser.add_argument(
        "--prompt",
        type=argparse.FileType("r"),
        required=True,
        help="Path to the file containing the prompt for Ollama",
    )
    parser.add_argument(
        "--beagle",
        type=argparse.FileType("r"),
        required=True,
        help="Path to the file containing existing Beagle analyses",
    )
    parser.add_argument(
        "--ollama-url", type=str, required=True, help="URL of the Ollama server"
    )
    parser.add_argument(
        "--ollama-model", type=str, help="Model to use in Ollama", default="llama3.2"
    )
    parser.add_argument(
        "--ollama-model-temperature", type=float,
        help="""
        Determine the creativity of the model in Ollama
        The higher the more creative, the lower the more factual.
        """,
        default=0.3
    )
    parser.add_argument(
        "--output-dir",
        type=str,
        required=True,
        help="Output directory for analysis result files",
    )
    parser.add_argument(
        "--governance",
        type=str,
        required=True,
        help="Path to the YAML file containing project and team information",
    )
    args = parser.parse_args()

    os.makedirs(args.output_dir, exist_ok=True)

    try:
        prompt = args.prompt.read()
    except Exception as e:
        print(f"Error reading prompt file '{args.prompt.name}':", e)
        return

    try:
        existing_beagle_projects = read_beagle_analysis(args.beagle.name)
    except Exception as e:
        print(f"Error reading Beagle file '{args.beagle.name}':", e)
        return

    if existing_beagle_projects is None:
        print("Cannot continue without existing Beagle analyses.")
        return

    try:
        team_to_projects = read_project_teams(args.governance)
    except Exception as e:
        print(f"Error reading YAML file '{args.governance}':", e)
        return

    if team_to_projects is None:
        print("Cannot continue without project and team information.")
        return

    team_analysis_results = defaultdict(list)
    projects = existing_beagle_projects

    for project_name, data in projects.items():
        team_name = next(
            (
                team
                for team, projects in team_to_projects.items()
                if project_name in projects
            ),
            "Unknown Team",
        )
        print(f"Analyzing project {project_name} (Team: {team_name})...")
        analysis_result = analyze_with_ollama(
            project_name, data, prompt, args.ollama_url, args.ollama_model, args.ollama_model_temperature
        )
        if analysis_result:
            list_of_data = [f"- {el}" for el in data]
            combined_result = (
                f"## Project: {project_name}\n{analysis_result}\n\nOccurrences Found:\n"
                + "\n".join(list_of_data)
                + "\n\n***\n"
            )
            team_analysis_results[team_name].append(combined_result)
        else:
            print(f"No analysis result for {project_name}")

    for team_name, analyses in team_analysis_results.items():
        team_filename = f"{team_name}.md"
        combined_analysis = "\n".join(analyses)
        store_analysis_result(
            team_filename,
            f"# Analysis for Team: {team_name}\n\n" + combined_analysis,
            args.output_dir,
        )


if __name__ == "__main__":
    main()
