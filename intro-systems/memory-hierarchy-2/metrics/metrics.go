package metrics

import (
	"encoding/csv"
	"log"
	"math"
	"os"
	"strconv"
	"time"
)

// 4 bytes
type UserId int
// 8 bytes (ptr, word)
type UserMap map[UserId]*User

// 12 bytes at least,  but probably 16 bytes w/ padding
type Address struct {
	// 8 bytes (ptr, word)
	fullAddress string
	// 4 bytes
	zip         int
}

// 16 bytes
type DollarAmount struct {
	dollars, cents uint64
}

// 40 bytes
type Payment struct {
	amount DollarAmount
	// 24 bytes
	time   time.Time
}

// 4 + 8 + 4 + 12 + 12
// 40 bytes
type User struct {
	id       UserId
	name     string
	age      int
	address  Address
	// 12 bytes
	payments []Payment
}

func AverageAge(ages []int) float64 {
	average, count := 0.0, 0.0
	for _, age := range ages {
		count += 1
		average += (float64(age) - average) / count
	}
	return average
}

func AveragePaymentAmount(users UserMap) float64 {
	average, count := 0.0, 0.0
	for _, u := range users {
		for _, p := range u.payments {
			count += 1
			amount := float64(p.amount.dollars) + float64(p.amount.cents)/100
			average += (amount - average) / count
		}
	}
	return average
}

// Compute the standard deviation of payment amounts
func StdDevPaymentAmount(users UserMap) float64 {
	mean := AveragePaymentAmount(users)
	squaredDiffs, count := 0.0, 0.0
	for _, u := range users {
		for _, p := range u.payments {
			count += 1
			amount := float64(p.amount.dollars) + float64(p.amount.cents)/100
			diff := amount - mean
			squaredDiffs += diff * diff
		}
	}
	return math.Sqrt(squaredDiffs / count)
}

func LoadData() UserMap {
	f, err := os.Open("users.csv")
	if err != nil {
		log.Fatalln("Unable to read users.csv", err)
	}
	reader := csv.NewReader(f)
	userLines, err := reader.ReadAll()
	if err != nil {
		log.Fatalln("Unable to parse users.csv as csv", err)
	}

	users := make(UserMap, len(userLines))
	for _, line := range userLines {
		id, _ := strconv.Atoi(line[0])
		name := line[1]
		age, _ := strconv.Atoi(line[2])
		address := line[3]
		zip, _ := strconv.Atoi(line[3])
		users[UserId(id)] = &User{UserId(id), name, age, Address{address, zip}, []Payment{}}
	}

	f, err = os.Open("payments.csv")
	if err != nil {
		log.Fatalln("Unable to read payments.csv", err)
	}
	reader = csv.NewReader(f)
	paymentLines, err := reader.ReadAll()
	if err != nil {
		log.Fatalln("Unable to parse payments.csv as csv", err)
	}

	for _, line := range paymentLines {
		userId, _ := strconv.Atoi(line[2])
		paymentCents, _ := strconv.Atoi(line[0])
		datetime, _ := time.Parse(time.RFC3339, line[1])
		users[UserId(userId)].payments = append(users[UserId(userId)].payments, Payment{
			DollarAmount{uint64(paymentCents / 100), uint64(paymentCents % 100)},
			datetime,
		})
	}

	return users
}

func LoadAges(users UserMap) []int {
	ages := make([]int, len(users))
	for i, u := range users {
		ages[i] = u.age
	}
	return ages
}
