package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(text string) FreqMap {
	frequencies := FreqMap{}
	for _, r := range text {
		frequencies[r]++
	}
	return frequencies
}

func FrequencyLine(text string, frequencies FreqMap) {
	for _, r := range text {
		frequencies[r]++
	}
}

func SequentialFrequency(texts []string) FreqMap {
	frequencies := make(FreqMap)
	for _, text := range texts {
		FrequencyLine(text, frequencies)
	}
	return frequencies
}

func FrequencyChannel(text string, ch chan FreqMap) {
	frequencies := FreqMap{}
	for _, r := range text {
		frequencies[r]++
	}
	ch <- frequencies
}

// ConcurrentFrequency counts the frequency of each rune in the given strings,
// by making use of concurrency.
func ConcurrentFrequency(texts []string) FreqMap {
	nb := len(texts)
	ch := make(chan FreqMap, nb)
	for _, text := range texts {
		go FrequencyChannel(text, ch)
	}

	frequencies := FreqMap{}
	for i := 0; i < nb; i++ {
		for k, v := range <-ch {
			frequencies[k] += v
		}
	}
	return frequencies
}
