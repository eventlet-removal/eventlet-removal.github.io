# eventlet-removal

Eventlet removal in an OpenStack context.
This repository aims to host the analyze of the various OpenStack deliverables.

## Tools

### Analyze Deliverables

The `tools/analyze.py` aim to analyze the list of all the Eventlet occurences found by
a doing a beagle research. This tool automatically analyze all the occurence of a
deliverable and create a report. That report tell us if we could already disabled
Eventlet by using an already existing parameter provided by the deliverable.

This tool create a report for each OpenStack team who have deliverables impacted by the
Eventlet removal.

This tool also aim to estimate the complexity of the removal for each impacted
deliverables.

This a tool based on AI, it consume:
- a list of occurence found by using beagle;
- an AI prompt to tell to the model what you want to do, the prompt used is included
  in this repository but you can design your own prompt;
- an ollama server url (e.g `http://0.0.0.0:11433/api/generate`);
- a list of OpenStack list extracted from the OpenStack governance repository
  (`wget https://raw.githubusercontent.com/openstack/governance/refs/heads/master/reference/projects.yaml`)
- an output dir to tell to the script where to store the generated reports

Example of usage:

```
# get the OpenStack governance details
$ wget https://raw.githubusercontent.com/openstack/governance/refs/heads/master/reference/projects.yaml
# retrieve all the Eventlet occurence in OpenStack, excluding comments
$ beagle search --ignore-comments \
    -f link --repo-pattern "openstack/*" 'eventlet' > eventlet-usage.txt
$ python3.11 ./tools/analyze.py \
    --prompt ./prompts/analyze.txt \
    --beagle eventlet-usage.txt \
    --ollama-url http://localhost:11434/api/generate \
    --ollama-model llama3.2 \
    --output-dir ./analyze \
    --governance projects.yaml
```
