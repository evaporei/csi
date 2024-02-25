package solution

// ScanOperator returns a pre-determined set of tuples.
type ScanOperator struct {
	tuples []Tuple
	idx    int
}

// NewScanOperator creates a new ScanOperator with the provided tuples.
func NewScanOperator(tuples []Tuple) Operator {
	return &ScanOperator{
		tuples: tuples,
		idx:    -1,
	}
}

// Next returns a boolean indicating whether there are more tuples to scan.
func (s *ScanOperator) Next() bool {
	s.idx++
	return s.idx < len(s.tuples)
}

// Execute returns the next tuple.
func (s *ScanOperator) Execute() Tuple {
	return s.tuples[s.idx]
}
