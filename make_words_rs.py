# Process each line
deny_words = []
deny_f = open("words_deny_list_3.txt")
deny_lines_raw = deny_f.readlines()
for i, line in enumerate(deny_lines_raw):
    if len(line) > 0:
        deny_words.append(line.strip().upper())
deny_f.close()

deny_f_2 = open("words_deny_list_4.txt")
deny_lines_raw = deny_f_2.readlines()
for i, line in enumerate(deny_lines_raw):
    if len(line) > 0:
        deny_words.append(line.strip().upper())

f = open("words.txt")
# Split the string by newline character
lines = f.readlines()[2:]

# Process each line
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