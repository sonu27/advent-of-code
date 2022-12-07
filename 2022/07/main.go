package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./2022/07/input.txt")
	if err != nil {
		panic(err)
	}

	root := NewDir("/", nil)
	var curDir *Dir

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		a := strings.Split(line, " ")

		if strings.HasPrefix(line, "$") {
			if a[1] == "cd" {
				if a[2] == "/" {
					curDir = root
				} else if a[2] == ".." {
					curDir = curDir.Parent
				} else {
					if v, ok := curDir.Dirs[a[2]]; ok {
						curDir = v
					} else {
						curDir = NewDir(a[2], curDir)
						curDir.Parent.Dirs[a[2]] = curDir
					}
				}
			}
			continue
		}

		// ls command
		if a[0] == "dir" {
			continue
		}

		size, _ := strconv.Atoi(a[0])
		curDir.Files[a[1]] = size
		curDir.Size += size
	}

	p1(root)
	p2(root)
}

func p1(root *Dir) {
	total := 0

	var walk func(*Dir)
	walk = func(dir *Dir) {
		for _, v := range dir.Dirs {
			walk(v)
			v.Parent.Size += v.Size
			if v.Size <= 100000 {
				total += v.Size
			}
		}
	}
	walk(root)

	fmt.Println("Part 1:", total)
}

func p2(root *Dir) {
	diskSize := 70000000
	spaceNeeded := 30000000
	spaceAvailable := diskSize - root.Size
	toDelete := spaceNeeded - spaceAvailable
	minFound := root.Size

	var walk func(*Dir)
	walk = func(dir *Dir) {
		for _, v := range dir.Dirs {
			walk(v)
		}
		if dir.Size >= toDelete && dir.Size < minFound {
			minFound = dir.Size
		}
	}
	walk(root)

	fmt.Println("Part 2:", minFound)
}

func NewDir(name string, parent *Dir) *Dir {
	return &Dir{
		Name:   name,
		Parent: parent,
		Dirs:   make(map[string]*Dir),
		Files:  make(map[string]int),
	}
}

type Dir struct {
	Name   string
	Parent *Dir
	Size   int
	Dirs   map[string]*Dir
	Files  map[string]int
}
