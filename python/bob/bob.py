def response(hey_bob):
    stripped_hey = hey_bob.strip()
    if len(stripped_hey) == 0 or stripped_hey.isspace():
        return "Fine. Be that way!"
    if stripped_hey.isupper() and stripped_hey[-1] == '?':
        return "Calm down, I know what I'm doing!"
    if stripped_hey.isupper():
        return "Whoa, chill out!"
    if stripped_hey[-1] == '?':
        return "Sure."
    return "Whatever."

