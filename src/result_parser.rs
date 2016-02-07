use json_flex;
use json_flex::JFObject;

#[derive(Clone)]
pub struct Rows {
    data: Option<Vec<JFObject>>,
}

/// A result rows representing type.
/// This type is usually generated by
/// [`ResultParser#into_raw()`](struct.ResultParser.html#method.into_row).
impl Rows {
    pub fn new(data: Option<Vec<JFObject>>) -> Rows {
        Rows { data: data }
    }

    /// Get columns in `Rows`.
    pub fn columns(&mut self) -> Option<Vec<JFObject>> {
        let popable = match self.data.clone() {
            Some(v) => v,
            None => return None,
        };
        let pop = match popable.clone().pop() {
            Some(v) => v,
            None => return None,
        };
        match pop.into_vec().clone() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResultParser {
    result: Box<JFObject>,
}

/// ResultParser
///
/// Groonga returns following array json:
///
/// success response:
/// `[[status, start_time, elapsed_time],
///   [[[matched_columns], [Array([column1, type1], ...)],
///     [Array([result1, result2, ...])]]]]`
///
/// error response:
/// `[[status, start_time, elapsed_time, error_information, ...]]`
impl ResultParser {
    pub fn new(json: String) -> ResultParser {
        ResultParser { result: json_flex::decode(json) }
    }

    /// Get raw response result.
    pub fn get_raw_object(&mut self) -> Box<JFObject> {
        self.result.clone()
    }

    /// Return header elements in response.
    ///
    /// # Panics
    ///
    /// Panics if response json is corrupted.
    pub fn get_header(&mut self) -> JFObject {
        self.result[0].clone()
    }

    /// Return status in response.
    ///
    /// # Panics
    ///
    /// Panics if response json is corrupted.
    pub fn status(&mut self) -> Option<&i64> {
        self.result[0][0].into_i64()
    }

    /// Return start time in response.
    ///
    /// # Panics
    ///
    /// Panics if response json is corrupted.
    pub fn start_time(&mut self) -> Option<&f64> {
        self.result[0][1].into_f64()
    }

    /// Return elapsed time in response.
    ///
    /// # Panics
    ///
    /// Panics if response json is corrupted.
    pub fn elapsed_time(&mut self) -> Option<&f64> {
        self.result[0][2].into_f64()
    }

    /// [nodoc]
    #[inline]
    fn matched_columns_num(&mut self) -> Option<i64> {
        let vectoizable = match self.result[1][0][0].into_vec().clone() {
            Some(elem) => elem,
            None => return None,
        };
        let pop = match vectoizable.clone().pop() {
            Some(pop) => pop,
            None => return None,
        };
        match pop.into_i64().clone() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    /// Return a number of matched columns in response.
    pub fn matched_columns(&mut self) -> Option<i64> {
        match self.status() {
            Some(&0) => self.matched_columns_num(),
            Some(_) => None,
            None => None,
        }
    }

    /// Get result in response.
    ///
    /// If request succeeded, it can get matched result array.
    /// Otherwise, one can get error messages.
    ///
    /// # Panics
    ///
    /// Panics if response json is corrupted.
    pub fn result(&mut self) -> Option<Vec<JFObject>> {
        match self.status() {
            Some(&0) => Some(vec![self.result[1][0].clone()]),
            Some(_) => Some(vec![self.result[0][3].clone()]),
            None => None,
        }
    }

    /// Convert to `Rows` type and return its type values.
    pub fn into_row(&mut self) -> Rows {
        Rows::new(self.result())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESPONSE: &'static str = "
    [[0,1452348610.39281,0.000101566314697266],
    [[[9],
     [[\"_id\",\"UInt32\"],[\"_key\",\"ShortText\"],[\"title\",\"ShortText\"]],
     [1,\"http://example.org/\",\"This is test record 1!\"],
     [2,\"http://example.net/\",\"test record 2.\"],
     [3,\"http://example.com/\",\"test test record three.\"],
     [4,\"http://example.net/afr\",\"test record four.\"],
     [5,\"http://example.org/aba\",\"test test test record five.\"],
     [6,\"http://example.com/rab\",\"test test test test record six.\"],
     [7,\"http://example.net/atv\",\"test test test record seven.\"],
     [8,\"http://example.org/gat\",\"test test record eight.\"],
     [9,\"http://example.com/vdw\",\"test test record nine.\"]]]]";

    #[test]
    fn parse_result() {
        let mut decode = ResultParser::new(RESPONSE.to_string());
        assert_eq!(&0, decode.status().unwrap());
        assert_eq!(&1452348610.39281, decode.start_time().unwrap());
        assert_eq!(&0.000101566314697266, decode.elapsed_time().unwrap());
        assert_eq!(9, decode.matched_columns().unwrap());
        let vec = decode.result().unwrap().pop().unwrap().unwrap_vec().clone();
        let expected = r#"Array([Integer(1), String("http://example.org/"), String("This is test record 1!")])"#.to_owned();
        assert_eq!(expected, format!("{:?}", vec[2]))
    }

    #[test]
    fn row_columns() {
        let mut decode = ResultParser::new(RESPONSE.to_string());
        let vec = decode.into_row().columns().unwrap();
        let expected = r#"Array([Integer(1), String("http://example.org/"), String("This is test record 1!")])"#.to_owned();
        assert_eq!(expected, format!("{:?}", vec[2]))
    }
}
