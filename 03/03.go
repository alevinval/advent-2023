package main

import (
	"fmt"
	"github.com/alevinval/advent-2023/utils"
	"strconv"
)

func main() {
	lines := utils.GetLines("03")
	engine := make([][]string, len(lines))

	for row, line := range lines {
		engine[row] = make([]string, len(line))
		for col, part := range line {
			engine[row][col] = string(part)
		}
	}

	sum, _ := gatherParts(engine, isSymbol)
	fmt.Printf("%d\n", sum)

	_, gearParts := gatherParts(engine, isGear)

	fmt.Printf("%d\n", gearRatio(engine, gearParts))
}

func gatherParts(engine [][]string, gatherFn func(string) bool) (int, [][]int) {
	gatheredParts := [][]int{}
	total := 0
	for x := range engine {
		chain := ""
		chainStart := 0
		for y := range engine[x] {
			curr := engine[x][y]
			if isDigit(curr) {
				chain += curr
				if len(chain) == 1 {
					chainStart = y
				}
				if y != len(engine[x])-1 {
					continue
				}
			}
			if chain != "" {
				if isPart(engine, x, chainStart, chain, gatherFn) {
					v := getDigit(chain)
					total += v
					gatheredParts = append(gatheredParts, []int{x, chainStart, v})
				}

				chain = ""
			}
		}
	}
	return total, gatheredParts
}

func isPart(engine [][]string, x int, yi int, chain string, gatherFn func(string) bool) bool {
	for idx := range chain {
		y := yi + idx

		for _, xo := range []int{-1, 0, 1} {
			xt := x + xo
			if xt < 0 || xt >= len(engine) {
				continue
			}

			for _, yo := range []int{-1, 0, 1} {
				yt := y + yo
				if yt < 0 || yt >= len(engine[xt]) {
					continue
				}

				if gatherFn(engine[xt][yt]) {
					return true
				}
			}
		}
	}

	return false
}

func gearRatio(engine [][]string, gearParts [][]int) int {
	gearRatio := 0
	for x := range engine {
		for y := range engine[x] {
			if engine[x][y] != "*" {
				continue
			}

			n, numbers := surroundingNumbers(x, y, gearParts)
			if n == 2 {
				acc := 1
				for _, number := range numbers {
					acc *= number
				}
				gearRatio += acc
			}
		}
	}
	return gearRatio
}

func surroundingNumbers(x int, y int, gearParts [][]int) (int, []int) {
	count := 0
	numbers := []int{}
	for _, part := range gearParts {
		xx, yy, vv := part[0], part[1], part[2]
		sv := strconv.Itoa(vv)
		if (xx-x) <= 1 && (xx-x) >= -1 {
			for idx := range sv {
				if (yy+idx-y) <= 1 && (yy+idx-y) >= -1 {
					count += 1
					numbers = append(numbers, vv)
					break
				}
			}
		}
	}
	return count, numbers
}

func isSymbol(value string) bool {
	return value != "." && !isDigit(value)
}

func isGear(value string) bool {
	return value == "*"
}

func isDigit(value string) bool {
	_, err := strconv.Atoi(value)
	return err == nil
}

func getDigit(value string) int {
	v, err := strconv.Atoi(value)
	if err != nil {
		panic("cannot parse integer")
	}
	return v
}
