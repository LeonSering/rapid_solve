use crate::objective::EvaluatedSolution;

pub(crate) enum SearchResult<S> {
    Improvement(EvaluatedSolution<S>),
    NoImprovement(EvaluatedSolution<S>),
}

impl<S> SearchResult<S> {
    pub(crate) fn unwrap(self) -> EvaluatedSolution<S> {
        match self {
            SearchResult::Improvement(solution) => solution,
            SearchResult::NoImprovement(solution) => solution,
        }
    }

    pub(crate) fn as_ref(&self) -> &EvaluatedSolution<S> {
        match self {
            SearchResult::Improvement(solution) => solution,
            SearchResult::NoImprovement(solution) => solution,
        }
    }
}
