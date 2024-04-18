import nltk
import ssl

try:
    _create_unverified_https_context = ssl._create_unverified_context
except AttributeError:
    pass
else:
    ssl._create_default_https_context = _create_unverified_https_context
nltk.download('words')
words = set(nltk.corpus.words.words())

f = open("words.txt")
# Split the string by newline character
lines = f.readlines()[2:]

# Process each line
newlines = []
for i, line in enumerate(lines):
    line = line.strip().lower()
    if not line.isalpha():
        continue
    if not (len(line) > 3 and len(line) < 5):
        continue
    if line.lower() not in words and line[:3] not in words:
        newlines.append(line)

with open("words_deny_list_2.txt", "w") as f:
    f.write('\n'.join(newlines))