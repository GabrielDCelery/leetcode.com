struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path_components: Vec<&str> =
            path.split("/").fold(Vec::new(), |mut acc, path_component| {
                match path_component {
                    ".." => {
                        acc.pop();
                    }
                    "" | "." => (),
                    _ => {
                        acc.push(path_component);
                    }
                };
                acc
            });
        format!("/{}", path_components.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use crate::simplify_path::Solution;

    #[test]
    fn case_1() {
        //Given
        let path = String::from("/home/");
        let expected = String::from("/home");

        //When
        let result = Solution::simplify_path(path);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let path = String::from("/home//foo/");
        let expected = String::from("/home/foo");

        //When
        let result = Solution::simplify_path(path);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        //Given
        let path = String::from("/home/user/Documents/../Pictures");
        let expected = String::from("/home/user/Pictures");

        //When
        let result = Solution::simplify_path(path);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_4() {
        //Given
        let path = String::from("/../");
        let expected = String::from("/");

        //When
        let result = Solution::simplify_path(path);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_5() {
        //Given
        let path = String::from("/.../a/../b/c/../d/./");
        let expected = String::from("/.../b/d");

        //When
        let result = Solution::simplify_path(path);

        //Then
        assert_eq!(result, expected);
    }
}
