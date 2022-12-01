package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"math/big"
	"strconv"
	"strings"
)

func main() {
	b, _ := ioutil.ReadFile("./13/input.txt")
	lines := strings.Split(string(b), "\n")
	num := s2i(lines[0])
	buses := strings.Split(lines[1], ",")

	part1(num, buses)
	part2(buses)
}

func part1(num int, s []string) {
	var buses []int
	for _, v := range s {
		if v != "x" {
			buses = append(buses, s2i(v))
		}
	}

	time := num
	for {
		for _, bus := range buses {
			if time%bus == 0 {
				fmt.Println(bus * (time - num))
				return
			}
		}
		time++
	}
}

type IndexedBus struct {
	Index int
	Bus   int
}

func part2(s []string) {
	var buses []IndexedBus
	for i, v := range s {
		if v != "x" {
			buses = append(buses, IndexedBus{
				Index: i,
				Bus:   s2i(v),
			})
		}
	}

	var bs []CrtEntry
	for i, v := range s {
		if v != "x" {
			bs = append(bs, CrtEntry{
				A: i,
				M: s2i(v) + i,
			})
		}
	}

	time := 0

	time = ChineseRemainderMany(bs)
	fmt.Println(time)

	//for {
	//	timeToSkipIfNoMatch := 1
	//	valid := true
	//
	//	for _, v := range buses {
	//		// A given bus will depart when the timestamp is evenly divisible by the bus ID
	//		if (time + v.Index) % v.Bus != 0 {
	//			// No match here; abort and we'll try the next potential timestamp
	//			valid = false
	//			break
	//		}
	//
	//		// This particular bus schedule matches, but we don't know if subsequent ones will. However, we
	//		// do know this when this particular schedule will match again, so let's keep track of that.
	//		// For example, if the first bus is Bus 7, we know it won't depart again for another 7 minutes,
	//		// so we'll skip ahead by 7 minutes and ignore the timestamps we know won't match.
	//		//
	//		// If we find a partial match where, say, 2 or 3 schedules match but not the whole thing, we
	//		// can calculate the next time those 2-3 schedules align by multiplying their values together;
	//		// worst case, we still have no match there, or best case we find yet another matching bus!
	//		//
	//		// For example, let's say we find a timestamp where the first two buses (7 and 11) align but none
	//		// of the others do; in that case, we know that buses 7 and 9 won't align again for another
	//		// 77 (7*11) minutes, so we'll skip ahead 77 minutes. Eventually we might find that now buses
	//		// 7, 11, and 13 align, but none others do. Well, that means the next time that these three buses
	//		// align will be in 7*11*13 minutes, so skip ahead by that much and try again there.
	//		//
	//		// This approach significantly speeds up the search and the speed improves the bigger your
	//		// partial match is!
	//		//
	//		// (Note that technically we'd need to find the LCM of those bus IDs, but luckily our inputs are
	//		// always prime numbers so the LCM will always equal the product of those IDs.)
	//		timeToSkipIfNoMatch *= v.Bus
	//	}
	//
	//	// Did we find a full match?
	//	if valid {
	//		fmt.Println(time)
	//		break
	//	}
	//
	//	time += timeToSkipIfNoMatch
	//}
}

func s2i(s string) int {
	i, _ := strconv.Atoi(s)
	return i
}

var one = big.NewInt(1)

func crt(a, n []*big.Int) (*big.Int, error) {
	p := new(big.Int).Set(n[0])
	for _, n1 := range n[1:] {
		p.Mul(p, n1)
	}
	var x, q, s, z big.Int
	for i, n1 := range n {
		q.Div(p, n1)
		z.GCD(nil, &s, n1, &q)
		if z.Cmp(one) != 0 {
			return nil, fmt.Errorf("%d not coprime", n1)
		}
		x.Add(&x, s.Mul(a[i], s.Mul(&s, &q)))
	}
	return x.Mod(&x, p), nil
}

// Solves x=a mod m; x=b mod n by using the chinese remainder theorem.
func ChineseRemainder(a, m, b, n int) int {
	s, t, _ := ExtendedGcd(m, n)
	return Mod(b*s*m+a*n*t, m*n)
}

// Represents an entry in the Extended Chinese Remainder Theorem
type CrtEntry struct {
	A, M int
}

// Solves the solution to x=(a1 mod m1); x=(a2 mod m2); x=...
//
// If len(eqs) == 0, it panics.
func ChineseRemainderMany(eqs []CrtEntry) int {
	if len(eqs) == 0 {
		panic("cannot have 0 entries to solve")
	}
	if len(eqs) == 1 {
		return Mod(eqs[0].A, eqs[0].M)
	}
	eqs2 := make([]CrtEntry, len(eqs))
	copy(eqs2, eqs)

	for i := 1; i < len(eqs2); i++ {
		x := ChineseRemainder(eqs2[i-1].A, eqs2[i-1].M, eqs2[i].A, eqs2[i].M)
		eqs2[i] = CrtEntry{x, eqs2[i-1].M * eqs2[i].M}
	}
	return eqs2[len(eqs2)-1].A
}

func solveCrtManyIntern(eqs []CrtEntry) int {
	f := eqs[0]
	s := eqs[1]
	x := ChineseRemainder(f.A, f.M, s.A, s.M)
	if len(eqs) == 2 {
		return x
	}
	eqs[1] = CrtEntry{x, f.M * s.M}
	return solveCrtManyIntern(eqs[1:])
}

var NoSolution = errors.New("no solution")

// Finds the least positive residue of a number
// in a given modulus. Note that this is very slightly
// different from the remainder (%) operator when working
// with negative numbers.
func Mod(a, m int) int {
	return (a%m + m) % m
}

// Solves the equation `ax=b mod n` for x. Note that
// if there are multiple LPR solutions that the
// lowest one is returned. If there are no solutions,
// then (0, NoSolution) is returned
func Solve(a, b, m int) (int, error) {
	gcd := Gcd(a, m)

	// If a and m are coprime, just multiply by the inverse
	if gcd == 1 {
		aInv, _, _ := ExtendedGcd(a, m)
		aInv = Mod(aInv, m)
		return Mod(aInv*b, m), nil
	}

	// If gcd divides b evenly, then solve a/d x = b/d mod m/d (d = gcd)
	if Mod(b, gcd) == 0 {
		ad := a / gcd
		bd := b / gcd
		nd := m / gcd
		return Solve(ad, bd, nd)
	}

	// else, no solution
	return 0, NoSolution
}

// Solves the equation x=a^b mod m. Note that there is not as large of a worry
// about overflowing, as a^b will not be calculated!
func Power(a, b, m int) int {
	// ints contains each multiple of a^(2^i) mod m where 2^i < b
	ints := []int{a}

	for j := 2; j < b; j *= 2 {
		last := ints[len(ints)-1]
		ints = append(ints, Mod(last*last, m))
	}

	for i := 0; i < len(ints); i++ {
		if b&^i == b {
			ints[i] = -1
		}
	}

	// Make a map of the powers of the ints.
	// So {7, 7, 4, 4, 7, 7} would become {7:4, 4:2}
	eq := make(map[int]int)
	for _, e := range ints {
		if e == -1 {
			continue
		}
		eq[e]++
	}

	// Simplify the map as much as possible
	modified := true
	for modified {
		modified = false
		next := make(map[int]int)

		for k, v := range eq {
			if v > 1 {
				lpr := Mod(k*k, m)
				next[k] = eq[k] - 2
				next[lpr] = eq[lpr] + 1
				modified = true
			}
		}
		for k, v := range next {
			eq[k] = v
		}
	}

	prod := 1
	for k, v := range eq {
		for i := 0; i < v; i++ {
			prod = Mod(prod*k, m)
		}
	}

	return prod
}

func Gcd(a, b int) int {

	next := a - a/b*b
	for next != 0 {
		oldB := b
		b = next
		next = a - (a / oldB * oldB)
		a = oldB
	}
	return b
}

// Finds x and y such that: Gcd(a, b) = ax + by. (By the extended euclidean algorithm)
//
// This implementation is based on
// https://en.wikibooks.org/wiki/Algorithm_Implementation/Mathematics/Extended_Euclidean_algorithm#Iterative_algorithm_3
func ExtendedGcd(a, b int) (x, y, gcd int) {
	x0, x1, y0, y1 := 1, 0, 0, 1

	for a != 0 {
		var q int
		q, b, a = b/a, a, b%a
		x0, x1 = x1, x0-q*x1
		y0, y1 = y1, y0-q*y1
	}
	return y0, x0, b
}
