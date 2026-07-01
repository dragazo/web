#[derive(Debug)]
pub struct Publication {
    pub year: usize,
    pub authors: &'static [&'static str],
    pub title: &'static str,
    pub venue: &'static str,
    pub link: &'static str,
}

pub static PUBLICATIONS: &[Publication] = &[
    Publication {
        year: 2025,
        authors: &["Devin Jean", "Suk Seo"],
        title: "On Error-Detecting Open-Locating-Dominating Sets",
        venue: "Ars Combinatoria",
        link: "https://doi.org/10.61091/ars165-05",
    },
    Publication {
        year: 2025,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Fault-Tolerant Detection Systems on the Infinite King Grid",
        venue: "Journal of Combinatorial Mathematics and Combinatorial Computing",
        link: "https://doi.org/10.61091/jcmcc128-24",
    },
    Publication {
        year: 2025,
        authors: &["Devin Jean", "Gordon Stein", "Brian Broll", "Akos Ledeczi"],
        title: "Easing the Block-to-Text Transition: A Scaffolded Approach to Learning Python",
        venue: "Proceedings of CompEd",
        link: "https://doi.org/10.1145/3736181.3747126",
    },
    Publication {
        year: 2025,
        authors: &["Devin Jean", "Jesse Turner", "Will Hedgecock", "Gyorgy Kalmar", "George Wittemyer", "Akos Ledeczi"],
        title: "Animal-Borne Adaptive Acoustic Monitoring",
        venue: "Journal of Sensors and Actuator Networks",
        link: "https://doi.org/10.3390/jsan14040066",
    },
    Publication {
        year: 2025,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Fault-Tolerant Locating-Dominating Sets with Error-Correction",
        venue: "Discrete Mathematics, Algorithms and Applications",
        link: "https://doi.org/10.1142/S1793830925500491",
    },
    Publication {
        year: 2024,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Optimal Error-Detection System for Identifying Codes",
        venue: "Networks",
        link: "https://doi.org/10.1002/net.22254",
    },
    Publication {
        year: 2024,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Open-Locating-Dominating Sets with Error Correction",
        venue: "Proceedings of the ACM Southeast Conference",
        link: "https://doi.org/10.1145/3603287.3651212",
    },
    Publication {
        year: 2024,
        authors: &["Shuchi Grover", "Devin Jean", "Brian Broll", "Veronica Catete", "Isabella Gransbury", "Akos Ledeczi", "Tiffany Barnes"],
        title: "Design of Tools and Learning Environments for Equitable Computer Science + Data Science Education",
        venue: "Book Chapter in Improving Equity in Data Science",
        link: "http://doi.org/10.4324/9781003364634-4",
    },
    Publication {
        year: 2024,
        authors: &["Devin Jean", "Gordon Stein", "Brian Broll", "Akos Ledeczi"],
        title: "Unleashing Creativity with Wireless Embedded Programming for Next-Generation Makers",
        venue: "Proceedings of EDULEARN",
        link: "https://doi.org/10.21125/edulearn.2024.0608",
    },
    Publication {
        year: 2024,
        authors: &["Tito Ebiwonjumi", "Will Hedgecock", "Devin Jean", "Gabriel Barnard", "Saman Kittani", "Brian Broll", "Akos Ledeczi"],
        title: "BeatBlox: A Visual Block-Based Approach to Integrating Music and Computer Science Education",
        venue: "Proceedings of EDULEARN",
        link: "https://doi.org/10.21125/edulearn.2024.0669",
    },
    Publication {
        year: 2024,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Fault-Tolerant Locating-Dominating Sets on the Infinite Tumbling Block Graph",
        venue: "Australasian Journal of Combinatorics",
        link: "https://ajc.maths.uq.edu.au/?page=get_volumes&volume=90",
    },
    Publication {
        year: 2024,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Fault-Tolerant Identifying Codes in Special Classes of Graphs",
        venue: "Discussiones Mathematicae Graph Theory",
        link: "https://doi.org/10.7151/dmgt.2465",
    },
    Publication {
        year: 2023,
        authors: &["Veronica Catete", "Isabella Gransbury", "Marnie Hill", "Devin Jean", "Brian Broll", "Akos Ledeczi", "Tiffany Barnes", "Shuchi Grover"],
        title: "Board 243: CS Frontiers: Module 4-A Software Engineering Curriculum for High School Females",
        venue: "ASEE Conferences",
        link: "https://doi.org/10.18260/1-2--42686",
    },
    Publication {
        year: 2023,
        authors: &["Devin Jean", "Suk Seo"],
        title: "On Redundant Locating-Dominating Sets",
        venue: "Discrete Applied Mathematics",
        link: "https://doi.org/10.1016/j.dam.2023.01.023",
    },
    Publication {
        year: 2023,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Optimal Error-Detecting Open-Locating-Dominating Set on the Infinite Triangular Grid",
        venue: "Discussiones Mathematicae Graph Theory",
        link: "http://doi.org/10.7151/dmgt.2374",
    },
    Publication {
        year: 2023,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Progress on Fault-Tolerant Locating-Dominating Sets",
        venue: "Discrete Mathematics Algorithms and Applications",
        link: "https://doi.org/10.1142/S179383092250080X",
    },
    Publication {
        year: 2023,
        authors: &["Gordon Stein", "Devin Jean", "Corey Brady", "Akos Ledeczi"],
        title: "Browser-Based Simulation for Novice-Friendly Classroom Robotics",
        venue: "Frontiers in Computer Science",
        link: "https://doi.org/10.3389/fcomp.2022.1031572",
    },
    Publication {
        year: 2023,
        authors: &["Devin Jean", "Shuchi Grover", "Akos Ledeczi", "Brian Broll"],
        title: "PhoneIoT for Teaching 'Internet of Things': Smartphones to Promote Accessible, Engaging, and Authentic Experiences",
        venue: "Proceedings of 17th International Conference of the Learning Sciences",
        link: "https://doi.org/10.22318/icls2023.870149",
    },
    Publication {
        year: 2022,
        authors: &["Devin Jean", "Brian Broll", "Gordon Stein", "Akos Ledeczi"],
        title: "Low-Friction Transition from Blocks to Python",
        venue: "Proceedings of the 2nd Annual Meeting of the International Society of the Learning Sciences",
        link: "https://2022.isls.org/proceedings/",
    },
    Publication {
        year: 2022,
        authors: &["Devin Jean", "Brian Broll", "Gordon Stein", "Akos Ledeczi"],
        title: "Utilizing Smartphones for Approachable IoT Education in K-12",
        venue: "Sensors",
        link: "https://doi.org/10.3390/s22249778",
    },
    Publication {
        year: 2022,
        authors: &["Corey Brady", "Brian Broll", "Gordon Stein", "Devin Jean", "Shuchi Grover", "Veronica Catete", "Tiffany Barnes", "Akos Ledeczi"],
        title: "Block-Based Abstractions and Expansive Services to Make Advanced Computing Concepts Accessible to Novices",
        venue: "Journal of Computer Languages",
        link: "https://doi.org/10.1016/j.cola.2022.101156",
    },
    Publication {
        year: 2022,
        authors: &["Devin Jean", "Suk Seo"],
        title: "Extremal Cubic Graphs for Fault-Tolerant Locating Domination",
        venue: "Theoretical Computer Science",
        link: "https://doi.org/10.1016/j.tcs.2022.03.014",
    },
    Publication {
        year: 2022,
        authors: &["Gordon Stein", "Isabella Gransbury", "Devin Jean", "Marnie Hill", "Veronica Catete", "Shuchi Grover", "Tiffany Barnes", "Brian Broll", "Akos Ledeczi"],
        title: "Engaging Female High School Students in the Frontiers of Computing",
        venue: "ASEE Annual Conference and Exposition",
        link: "https://doi.org/10.18260/1-2--42098",
    },
    Publication {
        year: 2021,
        authors: &["Devin Jean", "Brian Broll", "Gordon Stein", "Akos Ledeczi"],
        title: "Your Phone as a Sensor: Making IoT Accessible for Novice Programmers",
        venue: "IEEE Frontiers in Education Conference",
        link: "https://doi.org/10.1109/FIE49875.2021.9637272",

    },
    Publication {
        year: 2021,
        authors: &["Brian Broll", "Akos Ledeczi", "Gordon Stein", "Devin Jean", "Corey Brady", "Shuchi Grover", "Veronica Catete", "Tiffany Barnes"],
        title: "Removing the Walls around Visual Educational Programming Environments",
        venue: "IEEE Symposium on Visual Languages and Human-Centric Computing",
        link: "https://doi.org/10.1109/VL/HCC51201.2021.9576399",
    },
];

#[test]
fn test_publications() {
    let mut links = std::collections::BTreeSet::new();
    for x in PUBLICATIONS {
        if !links.insert(x.link) {
            panic!("duplicate link: {}", x.link);
        }
        if x.authors.iter().find(|&&x| x == "Devin Jean").is_none() {
            panic!("missing me: {x:?}");
        }
    }
}
