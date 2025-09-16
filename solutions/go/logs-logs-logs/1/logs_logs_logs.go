package logs

import "unicode/utf8"

// Application identifies the application emitting the given log.
func Application(log string) string {
	rr, _ := utf8.DecodeRuneInString("❗")
	sr, _ := utf8.DecodeRuneInString("🔍")
	wr, _ := utf8.DecodeRuneInString("☀")
	for _, ch := range log {
		switch ch {
		case rr:
			return "recommendation"
		case sr:
			return "search"
		case wr:
			return "weather"
		}
	}
	return "default"
}

// Replace replaces all occurrences of old with new, returning the modified log
// to the caller.
func Replace(log string, oldRune, newRune rune) string {
	runes := []rune(log)
	for idx, ch := range runes {
		if ch == oldRune {
			runes[idx] = newRune
		}
	}
	return string(runes)
}

// WithinLimit determines whether or not the number of characters in log is
// within the limit.
func WithinLimit(log string, limit int) bool {
	return utf8.RuneCountInString(log) <= limit
}
