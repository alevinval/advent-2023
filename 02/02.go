package main

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

type game struct {
	id      int
	maxHist map[string]int
}

func main() {
	f, err := os.Open("02/02.txt")
	if err != nil {
		panic(err)
	}
	data, err := io.ReadAll(f)
	if err != nil {
		panic(err)
	}

	possibleGames := 0
	power := 0
	for _, line := range strings.Split(string(data), "\n") {
		if line == "" {
			break
		}
		game := parseLine(line)
		power += game.Power()
		if isGamePossible(game) {
			possibleGames += game.id
		}
	}

	fmt.Printf("possible games: %d\n", possibleGames)
	fmt.Printf("power: %d\n", power)
}

func parseLine(line string) *game {
	split := strings.Split(line, ":")
	gameStr, remainingStr := split[0], split[1]
	roundsStr := strings.Split(remainingStr, ";")

	maxHist := map[string]int{}
	for _, roundStr := range roundsStr {
		for _, cubeStr := range strings.Split(roundStr, ",") {
			cubeStr = strings.Trim(cubeStr, " ")
			cube := strings.Split(cubeStr, " ")
			color := cube[1]
			amount, _ := strconv.Atoi(cube[0])
			if maxHist[color] < amount {
				maxHist[color] = amount
			}
		}
	}

	id, _ := strconv.Atoi(strings.Split(gameStr, " ")[1])
	return &game{
		id,
		maxHist,
	}
}

func isGamePossible(g *game) bool {
	return g.maxHist["red"] <= 12 && g.maxHist["green"] <= 13 && g.maxHist["blue"] <= 14
}

func (g *game) Power() int {
	return g.maxHist["red"] * g.maxHist["green"] * g.maxHist["blue"]
}
