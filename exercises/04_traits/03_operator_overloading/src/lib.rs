use std::cmp::PartialEq;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: Implement the `PartialEq` trait for `Ticket`.

impl PartialEq for Ticket {
    fn eq(&self, other: &Ticket) -> bool {
        // It can be defined as: fn eq(&self, other: &self) -> bool {
        // self.title == other.title && self.description == other.description && self.status == other.status

        // This can be implemented this way too:
        // This way if the structure changes, compiler will error.
        // This is called destructuring 
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title, // this changes the name of the filed in the struct.
            description: other_description,
            status: other_status,
        } = other;
        title == other_title && description == other_description && status == other_status
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 == ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert!(ticket1 != ticket2);
    }
}
