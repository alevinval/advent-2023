package main

import (
	"fmt"
	"strings"

	"os"

	"github.com/alevinval/advent-2023/utils"
)

type Card struct {
	id      int
	n       int
	winning map[int]struct{}
	numbers map[int]struct{}
}

func main() {
	lines := utils.GetLines("04")

	cardMap := map[int]*Card{}
	utils.ForLine(lines, func(line string) {
		c := getCard(line)
		cardMap[c.id] = c
	})

	utils.ForLine(lines, func(line string) {
		id := getId(line)
		original := cardMap[id]

		for i := 1; i <= original.PointsPart2(); i++ {
			idx := original.id + i
			copied, ok := cardMap[idx]
			if ok {
				copied.n += original.n
				fmt.Printf("card %d added %d to card %d (%d)\n", id, original.n, idx, copied.n)
			}
		}
	})

	total := 0
	for _, card := range cardMap {
		total += card.n
	}

	fmt.Fprintf(os.Stdout, "Total Points: %d\n", []any{total}...)
}

func getId(line string) int {
	split := strings.Split(line, ":")
	return utils.GetDigit(strings.Trim(strings.SplitN(split[0], " ", 2)[1], " "))
}

func getCard(line string) *Card {
	split := strings.Split(line, ":")
	split = strings.Split(split[1], "|")
	winning := map[int]struct{}{}
	for _, w := range utils.TSplit(split[0], " ") {
		if w == "" {
			continue
		}
		winning[utils.GetDigit(w)] = struct{}{}
	}
	numbers := map[int]struct{}{}
	for _, w := range utils.TSplit(split[1], " ") {
		if w == "" {
			continue
		}
		numbers[utils.GetDigit(w)] = struct{}{}
	}
	return &Card{
		getId(line),
		1,
		winning,
		numbers,
	}
}

func (c *Card) PointsPart1() int {
	points := 0
	for n := range c.numbers {
		_, ok := c.winning[n]
		if ok {
			if points == 0 {
				points = 1
			} else {
				points *= 2
			}
		}
	}
	return points
}

func (c *Card) PointsPart2() int {
	points := 0
	for n := range c.numbers {
		_, ok := c.winning[n]
		if ok {
			points += 1
		}
	}
	return points
}
