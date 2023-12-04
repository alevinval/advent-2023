package utils

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func GetLines(day string) []string {
	f, err := os.Open(fmt.Sprintf("../inputs/%s.txt", day))
	if err != nil {
		panic(err)
	}
	data, err := io.ReadAll(f)
	if err != nil {
		panic(err)
	}

	return strings.Split(string(data), "\n")
}

func ForLine(lines []string, fn func(line string)) {
	for _, line := range lines {
		if line == "" {
			continue
		}
		fn(line)
	}
}

func GetDigit(value string) int {
	v, err := strconv.Atoi(value)
	if err != nil {
		panic(fmt.Sprintf("cannot parse integer: %q", value))
	}
	return v
}

func TSplit(value string, sep string) []string {
	return strings.Split(strings.Trim(value, " "), " ")
}
