package solution

// LimitOperator limits the number of tuples returned based on the provided limit.
type LimitOperator struct {
	// Arguments.
	limit int
	child Operator

	// State.
	numReturned int
}

// NewLimitOperator creates a new LimitOperator.
func NewLimitOperator(limit int, child Operator) Operator {
	return &LimitOperator{
		limit:       limit,
		child:       child,
		numReturned: 0,
	}
}

// Next returns whether the the LimitOperator has another value to return.
func (l *LimitOperator) Next() bool {
	hasNext := l.numReturned < l.limit && l.child.Next()
	if hasNext {
		l.numReturned++
	}
	return hasNext
}

// Execute returns the next tuple. Execute should only be called after a
// call to Next() that returned true.
func (l *LimitOperator) Execute() Tuple {
	return l.child.Execute()
}
