f = open("words.txt")
# Split the string by newline character
lines = f.readlines()[2:]

# Process each line
newlines = []
for i, line in enumerate(lines[:25000]):
   line = line.strip()
   add = True
   for c in line:
       if not (c.isalpha() and c.upper() >= "A" and c.upper() <= "Z"):
            add = False

   if not add:
       continue
   if len(line) > 0:
        newlines.append("\t\t\"" + line.upper()+ "\",")  # Join words with commas

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