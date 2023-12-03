package main

import (
	"strings"

	"github.com/alevinval/advent-2023/utils"
)

var (
	dictionary = map[string]string{
		"one":   "1",
		"two":   "2",
		"three": "3",
		"four":  "4",
		"five":  "5",
		"six":   "6",
		"seven": "7",
		"eight": "8",
		"nine":  "9",
	}
	overlaps = map[string]string{
		"oneight":   "18",
		"twone":     "21",
		"threeight": "38",
		"fiveight":  "58",
		"eightwo":   "82",
		"eighthree": "83",
	}
)

func main() {
	lines := utils.GetLines("01")
	total := 0
	for idx := range lines {
		total += recoverCode(lines[idx])
	}
	println(total)
}

func recoverCode(line string) int {
	line = normalize(line)
	min := 0
	code := 0
	for i := 0; i < len(line); i++ {
		char := line[i]
		if isNumeric(char) {
			min = i
			code = (int)(char-48) * 10
			break
		}
	}
	for i := len(line) - 1; i >= min; i-- {
		char := line[i]
		if isNumeric(char) {
			code += (int)(char - 48)
			break
		}
	}
	return code
}

func isNumeric(c byte) bool {
	return c >= 48 && c <= 57
}

func normalize(line string) string {
	for from, to := range overlaps {
		line = strings.ReplaceAll(line, from, to)
	}
	for from, to := range dictionary {
		line = strings.ReplaceAll(line, from, to)
	}
	return line
}
