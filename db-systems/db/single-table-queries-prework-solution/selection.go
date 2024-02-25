package solution

// SelectionOperator filters tuples from its child based on the provided BinaryExpression.
type SelectionOperator struct {
	// Provided.
	exp   BinaryExpression
	child Operator

	// State.
	curr Tuple
}

// NewSelectionOperator creates a new SelectionOperator.
func NewSelectionOperator(exp BinaryExpression, child Operator) Operator {
	return &SelectionOperator{
		exp:   exp,
		child: child,
	}
}

// Next returns a boolean indicating whether the SelectionOperator has an additional value.
//
// Note that calling Next() once on the SelectionOperator may consume an arbitrary number
// of tuples from its child until one is found that passes the provided BinaryExpression
// or all the child tuples are exhausted.
func (f *SelectionOperator) Next() bool {
	for f.child.Next() {
		tuple := f.child.Execute()
		if f.exp.Execute(tuple) {
			f.curr = tuple
			return true
		}
	}

	return false
}

// Execute returns the Tuple that passed the BinaryExpression. Should only be called after
// a call to Next() that returns true.
func (f *SelectionOperator) Execute() Tuple {
	return f.curr
}
