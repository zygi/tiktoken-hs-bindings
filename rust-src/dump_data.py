import base64
import copy
import sys
import tiktoken_ext.openai_public
import json

def encode(obj):
    obj = copy.deepcopy(obj)
    obj['mergeable_ranks'] = {base64.b64encode(k).decode('utf-8'): v for k, v in obj['mergeable_ranks'].items()}
    # obj['special_tokens'] = {base64.b64encode(k.).decode('utf-8'): v for k, v in obj['special_tokens'].items()}
    return obj

if __name__ == '__main__':
    params = getattr(tiktoken_ext.openai_public, sys.argv[1])()
    with open(sys.argv[2], 'w') as f:
        json.dump(encode(params), f)


