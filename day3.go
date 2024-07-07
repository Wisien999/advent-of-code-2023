package main

import (
	"fmt" 
	"strconv"
	"os"
	"log"
	"bufio"
)

func find_end_of_num(schematic []string, i, j int) int {
	last_j := j
	for ; j < len(schematic[i]); j++ {
		if is_num(schematic, i, j) {
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
		if is_num(schematic, i, j) {
			last_j = j
		} else {
			break
		}
	}

	return last_j
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
			if !is_num(schematic, i+x, j+y) && schematic[i + x][j + y] != '.' {
				return true
			}
		}
	}

	return false
}

func is_num(schematic []string, i, j int) bool {
	return schematic[i][j] >= '0' && schematic[i][j] <= '9'
}

func parse_num(schematic []string, i, j int) (int, int, int, error) {
	start := find_start_of_num(schematic, i, j)
	end := find_end_of_num(schematic, i, j)
	num, err := strconv.Atoi(schematic[i][start:end + 1])
	if err != nil {
		return 0, 0, 0, fmt.Errorf("can't create number from %d to %d (current cell (%d, %d)): %w", start, end, i, j, err)
	}

	return num, start, end, nil
}

func adjacent_nums(schematic []string, i, j int) ([]int, error) {
	nums := make([]int, 0)

	for x := -1; x <= 1; x++ {
		for y := -1; y <= 1; y++ {
			new_i := i + x
			new_j := j + y

			if x == 0 && y == 0 {
				continue
			}
			if new_i < 0 || new_i >= len(schematic) || new_j < 0 || new_j >= len(schematic[0]) {
				continue
			}
			if is_num(schematic, new_i, new_j) {
				num, _, end, err := parse_num(schematic, new_i, new_j)
				if err != nil {
					return nil, err
				}

				y = end - j

				nums = append(nums, num)
			}
		}
	}

	return nums, nil
}


func solve_part_1(schematic []string) (int, error) {
	sum := 0

	for i := 0; i < len(schematic); i++ {
		for j := 0; j < len(schematic[i]); j++ {
			if schematic[i][j] >= '0' && schematic[i][j] <= '9' && adjacent_to_part(schematic, i, j) {
				start := find_start_of_num(schematic, i, j)
				end := find_end_of_num(schematic, i, j)

				num_str := schematic[i][start: end + 1]
				num, err := strconv.Atoi(num_str)
				if err != nil {
					return 0, fmt.Errorf("can't create number from %d to %d (current cell (%d, %d)): %w", start, end, i, j, err)

				}

				sum += num

				j = end
			}
		}
	}


	return sum, nil
}


type NotGearError struct {
    row     int
    column  int
    Message string
}

func (e *NotGearError) Error() string {
    return fmt.Sprintf("NotGearError on cell (%d, %d): %s", e.row, e.column, e.Message)
}

func createNotGearError(row, column int, message string) *NotGearError {
	return &NotGearError{row, column, message}
}

func gear_ratio(schematic []string, i, j int) (int, error) {
	if schematic[i][j] != '*' {
		return 0, createNotGearError(i, j, "cell is not a gear symbol")
	}

	adjacent_nums, err := adjacent_nums(schematic, i, j)
	if err != nil {
		return 0, err
	}

	if len(adjacent_nums) != 2 {
		return 0, createNotGearError(i, j, "wrong number of adjacent numbers")
	}

	return adjacent_nums[0] * adjacent_nums[1], nil
}

func sum_gear_ratios(schematic []string) (int, error) {
	sum := 0
	for i := 0; i < len(schematic); i++ {
		for j := 0; j < len(schematic[i]); j++ {
			ratio, err := gear_ratio(schematic, i, j)
			if err != nil {
				switch err.(type) {
					case *NotGearError:
						ratio = 0
					default:
						return 0, err
				}
			}

			sum += ratio
		}
	}
	return sum, nil
}


func main() {
	file, err := os.Open("day3_data.txt")
	if err != nil {
		log.Fatal(err)
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
	// fmt.Println(fileLines)


	ans, err := sum_gear_ratios(fileLines)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println("Answer: ", ans)
}
