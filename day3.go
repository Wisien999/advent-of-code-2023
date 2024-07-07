package main

import (
	"fmt" 
	"strconv"
	"os"
	"log"
	"bufio"
)

func make2D(rows, cols int) [][]bool {
	a := make([][]bool, rows)
	for i := range a {
		a[i] = make([]bool, cols)
	}
	return a
}

func find_end_of_num(schematic []string, i, j int) int {
	last_j := j
	for ; j < len(schematic[i]); j++ {
		if schematic[i][j] >= '0' && schematic[i][j] <= '9' {
			last_j = j
		} else {
			break
		}
	}

	return last_j
}

func find_start_of_num(schematic []string, i, j int) int {
	last_j := j
	for ; j >= 0; j-- {
		if schematic[i][j] >= '0' && schematic[i][j] <= '9' {
			last_j = j
		} else {
			break
		}
	}

	return last_j
}

type Point struct {
	row int
	column int
}

func adjacent_to_part(schematic []string, i, j int) bool {
	for x := -1; x <= 1; x++ {
		for y := -1; y <= 1; y++ {
			if x == 0 && y == 0 {
				continue
			}
			if i + x < 0 || i + x >= len(schematic) || j + y < 0 || j + y >= len(schematic[0]) {
				continue
			}
			if !(schematic[i + x][j + y] >= '0' && schematic[i + x][j + y] <= '9') && schematic[i + x][j + y] != '.' {
				return true
			}
		}
	}

	return false
}


func solve(schematic []string) (int, error) {
	sum := 0

	for i := 0; i < len(schematic); i++ {
		for j := 0; j < len(schematic[i]); j++ {
			if schematic[i][j] >= '0' && schematic[i][j] <= '9' && adjacent_to_part(schematic, i, j) {
				start := find_start_of_num(schematic, i, j)
				end := find_end_of_num(schematic, i, j)

				num_str := schematic[i][start: end + 1]
				num, err := strconv.Atoi(num_str)
				if err != nil {
					fmt.Println(err)
					return 0, fmt.Errorf("can't create number from %d to %d (current cell (%d, %d)): %w", start, end, i, j, err)

				}

				sum += num

				j = end
			}
		}
	}


	return sum, nil
}

func main() {
	file, err := os.Open("day3_data.txt")
	if err != nil {
		log.Fatal(err)
		fmt.Println(err)
		return
	}
	defer func() {
		fmt.Println("Closing file")
		if err = file.Close(); err != nil {
		    log.Fatal(err)
		}
	}()

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)
	var fileLines []string

	for fileScanner.Scan() {
		fileLines = append(fileLines, fileScanner.Text())
	}
	fmt.Println(fileLines)


	ans, err := solve(fileLines)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println("Answer: ", ans)
}
