package metrics

import (
	"encoding/csv"
	"log"
	"math"
	"os"
	"strconv"
)

type UserData struct {
	ages     []uint8
	payments []uint32
}

func AverageAge(users UserData) float64 {
	sum := uint64(0)
	for _, age := range users.ages {
		sum += uint64(age)
	}
	count := len(users.ages)
	return float64(sum) / float64(count)
}

func AveragePaymentAmount(users UserData) float64 {
	sum := uint64(0)
	for _, p := range users.payments {
		sum += uint64(p)
	}
	count := len(users.payments)
	return 0.01 * float64(sum) / float64(count)
}

// Compute the standard deviation of payment amounts
// Variance[X] = E[X^2] - E[X]^2
func StdDevPaymentAmount(users UserData) float64 {
	// Using uint64 here would cause overflow
	sumSquare, sum := float64(0), float64(0)
	for _, p := range users.payments {
		x := float64(p) * 0.01
		sumSquare += x * x
		sum += x
	}
	count := len(users.payments)
	avgSquare := sumSquare / float64(count)
	avg := sum / float64(count)
	return math.Sqrt(avgSquare - avg*avg)
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

	payments := make([]uint32, len(paymentLines))
	for i, line := range paymentLines {
		paymentCents, _ := strconv.ParseUint(line[0], 10, 32)
		payments[i] = uint32(paymentCents)
	}

	return UserData{ages, payments}
}
