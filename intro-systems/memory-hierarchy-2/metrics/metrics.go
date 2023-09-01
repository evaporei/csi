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

type UserData struct {
	ages []uint8
	amounts []DollarAmount
}

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

func AverageAge(users *UserData) float64 {
	average := 0.0
	for _, age := range users.ages {
		average += float64(age)
	}
	return average / float64(len(users.ages))
}

func AveragePaymentAmount(users *UserData) float64 {
	average, count := 0.0, 0.0
	for _, dollarAmount := range users.amounts {
		count += 1
		amount := float64(dollarAmount.dollars) + float64(dollarAmount.cents)/100
		average += (amount - average) / count
	}
	return average
}

// Compute the standard deviation of payment amounts
func StdDevPaymentAmount(users *UserData) float64 {
	mean := AveragePaymentAmount(users)
	squaredDiffs, count := 0.0, 0.0
	for _, dollarAmount := range users.amounts {
		count += 1
		amount := float64(dollarAmount.dollars) + float64(dollarAmount.cents)/100
		diff := amount - mean
		squaredDiffs += diff * diff
	}
	return math.Sqrt(squaredDiffs / count)
}

func LoadData() *UserData {
	f, err := os.Open("users.csv")
	if err != nil {
		log.Fatalln("Unable to read users.csv", err)
	}
	reader := csv.NewReader(f)
	userLines, err := reader.ReadAll()
	if err != nil {
		log.Fatalln("Unable to parse users.csv as csv", err)
	}

	ages := make([]uint8, len(userLines))
	for i, line := range userLines {
		age, _ := strconv.Atoi(line[2])
		ages[i] = uint8(age)
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

	var amounts []DollarAmount
	for _, line := range paymentLines {
		paymentCents, _ := strconv.Atoi(line[0])
		amounts = append(amounts, DollarAmount{uint64(paymentCents / 100), uint64(paymentCents % 100)})
	}

	return &UserData {
		ages,
		amounts,
	}
}
