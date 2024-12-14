import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./ui/Card";

const posts = [
  {
    title:
      "Automate UX Performance Testing with Puppeteer and the Chrome Dev Tools Performance Timeline",
    id: 1,
    user: "Torre Taylor",
    date: "12/20/2020",
    url: "https://dev.to/tostaylo/automate-ux-performance-testing-with-puppeteer-and-the-chrome-dev-tools-performance-timeline-aah",
  },
  {
    title: "Lessons Learned After Building a Front-End Library with Rust",
    id: 2,
    user: "Torre Taylor",
    date: "2/01/2021",
  },
];

export function Posts() {
  return (
    <>
      {posts.map((post) => (
        <div className="my-8">
          <a href={`/posts/post/${post.id}`}>
            <Card key={post.id}>
              <CardHeader>
                <CardTitle>{post.title}</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription>
                  by {post.user} on {post.date}
                  {post.url && (
                    <a className="m-8" href={post.url}>
                      Read On
                    </a>
                  )}
                </CardDescription>
              </CardContent>
            </Card>
          </a>
        </div>
      ))}
    </>
  );
}
