package utils

import (
	"fmt"
	"io"
	"os"
	"strings"
)

func GetLines(day string) []string {
	f, err := os.Open(fmt.Sprintf("%s/%s.txt", day, day))
	if err != nil {
		panic(err)
	}
	data, err := io.ReadAll(f)
	if err != nil {
		panic(err)
	}

	return strings.Split(string(data), "\n")
}
