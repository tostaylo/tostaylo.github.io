import { useState, useEffect } from "react";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function Post({ params }: { params: any }) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [post, setPost] = useState("" as any);

  useEffect(() => {
    fetch("/assets/posts/html/post." + params.id + ".html")
      .then((res) => res.text())
      .then((data) => {
        console.log(data);
        setPost(data);
      });
  }, [params.id]);

  return (
    <>
      <div dangerouslySetInnerHTML={{ __html: post }}></div>
    </>
  );
}
