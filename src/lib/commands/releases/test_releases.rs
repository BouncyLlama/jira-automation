#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use crate::Cli;
    use crate::lib::commands::releases::{assemble_query, execute_list_releases, PaginatedStuff, Release, ListReleasesArgs, page_loop};
    use crate::lib::util::Format;
    use httpmock::prelude::*;
    use serde_json::json;

    #[test]
    fn list() -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Cli {
            authToken: "".to_string(),

            output_format: Format::csv,
            userEmail: "".to_string(),
            baseJiraUrl: "asdf".to_string(),
            command: None,
        };
        let args = ListReleasesArgs {
            project: "foo".to_string(),
            filter: None,
            unpaginate: false,
            page_size: 50,
            page_start_idx: 0,
        };
        let (url, args) = assemble_query(&ctx, &args);
        assert!(url.contains("asdf") && url.contains("foo"));
        assert_eq!(2, args.len());
        assert!(args.contains_key("startAt"));
        assert!(args.contains_key("maxResults"));
        assert_eq!("50", args.get("maxResults").unwrap());
        assert_eq!("0", args.get("startAt").unwrap());

        Ok(())
    }

    #[test]
    fn pageloop() -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Cli {
            authToken: "".to_string(),

            output_format: Format::csv,
            userEmail: "".to_string(),
            baseJiraUrl: "asdf".to_string(),
            command: None,
        };
        let args = ListReleasesArgs {
            project: "foo".to_string(),
            filter: None,
            unpaginate: true,
            page_size: 50,
            page_start_idx: 0,
        };
        let result = page_loop(100, &args);
        assert_eq!(2, result.len());
        assert_eq!("0", result.get(0).unwrap().get("startAt").unwrap());
        assert_eq!("50", result.get(1).unwrap().get("startAt").unwrap());

        Ok(())
    }
}