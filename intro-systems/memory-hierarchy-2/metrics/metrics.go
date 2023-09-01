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
	amounts []uint32 // paymentCents
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

func AverageAge(users UserData) float64 {
	average := uint64(0)
	for _, age := range users.ages {
		average += uint64(age)
	}
	return float64(average) / float64(len(users.ages))
}

func AveragePaymentAmount(users UserData) float64 {
	sum := uint64(0)
	for _, paymentCents := range users.amounts {
		sum += uint64(paymentCents)
	}
	return 0.01 * float64(sum) / float64(len(users.amounts))
}

// Compute the standard deviation of payment amounts
// Variance[X] = E[X^2] - E[X]^2
func StdDevPaymentAmount(users UserData) float64 {
	sumSquare, sum := 0.0, 0.0
	for _, paymentCents := range users.amounts {
		x := float64(paymentCents) * 0.01
		sumSquare += x * x
		sum += x
	}
	count := float64(len(users.amounts))
	avgSquare := sumSquare / count
	avg := sum / count
	return math.Sqrt(avgSquare - avg * avg)
}

func LoadData() UserData {
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

	amounts := make([]uint32, len(paymentLines))
	for i, line := range paymentLines {
		paymentCents, _ := strconv.ParseUint(line[0], 10, 32)
		amounts[i] = uint32(paymentCents)
	}

	return UserData {
		ages,
		amounts,
	}
}
