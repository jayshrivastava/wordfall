# get deny list
deny_words = []
deny_f = open("words_deny_list.txt")
deny_lines_raw = deny_f.readlines()
for i, line in enumerate(deny_lines_raw):
    if len(line) > 0:
        deny_words.append(line.strip().upper())
deny_f.close()

# iterate through all words + extra words
f = open("words.txt")
lines = f.readlines()[2:]

extra_words = []
extra_f = open("words_extra.txt")
extra_lines_raw = extra_f.readlines()
for i, line in enumerate(extra_lines_raw):
    if len(line) > 0:
        lines.append(line)
extra_f.close()

# Process each line
lines.sort()
newlines = []
for i, line in enumerate(lines):
   line = line.strip()
   add = True
   for c in line:
       if not (c.isalpha() and c.upper() >= "A" and c.upper() <= "Z"):
            add = False

   if len(line) > 7 or len(line) < 3:
       add = False

   if line.upper() in deny_words:
       add = False

   if not add:
       continue

   newlines.append("\t\t\"" + line.upper()+ "\",")  # Join words with commas

f.close()

# Join the lines with newline character and return

# Write the result to "out.txt" file
with open("src/words.rs", "w") as f:
    f.write("""pub fn make_words() -> Vec<&'static str> {
    return vec![
    """)
    f.write('\n'.join(newlines))
    f.write("""
    ]
}
    """)
