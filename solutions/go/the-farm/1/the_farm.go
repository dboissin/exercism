package thefarm

import (
	"errors"
	"fmt"
)

type InvalidCowsError struct {
	cows int
	msg  string
}

func (e *InvalidCowsError) Error() string {
	return fmt.Sprintf("%d cows are invalid: %s", e.cows, e.msg)
}

func DivideFood(calculator FodderCalculator, nbCows int) (float64, error) {
	amount, err := calculator.FodderAmount(nbCows)
	if err != nil {
		return 0, err
	}
	factor, err := calculator.FatteningFactor()
	if err != nil {
		return 0, err
	}
	return amount * factor / float64(nbCows), nil
}

func ValidateInputAndDivideFood(calculator FodderCalculator, nbCows int) (float64, error) {
	if nbCows > 0 {
		return DivideFood(calculator, nbCows)
	} else {
		return 0, errors.New("invalid number of cows")
	}
}

func ValidateNumberOfCows(nbCows int) error {
	if nbCows < 0 {
		return &InvalidCowsError{nbCows, "there are no negative cows"}
	} else if nbCows == 0 {
		return &InvalidCowsError{nbCows, "no cows don't need food"}
	} else {
		return nil
	}
}
