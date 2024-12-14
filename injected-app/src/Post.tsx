import { useState, useEffect } from "react";

export function Post({ params }: { params: { id: string } }) {
  const [post, setPost] = useState("");

  useEffect(() => {
    fetch("/assets/posts/html/post." + params.id + ".html")
      .then((res) => res.text())
      .then((data) => {
        setPost(data);
      });
  }, [params.id]);

  return (
    <>
      <div dangerouslySetInnerHTML={{ __html: post }}></div>
    </>
  );
}
