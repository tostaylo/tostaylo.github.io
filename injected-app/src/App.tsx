import { Route, Switch, Router } from "wouter";

import "./App.css";
import { Post } from "./Post";

export default function App() {
  const base = window.location.pathname.includes("posts")
    ? "/posts"
    : "/projects";

  return (
    <Router base={base}>
      <Switch>
        <Route path="/post/:id">{(params) => <Post params={params} />}</Route>

        <Route>404: No such page!</Route>
      </Switch>
    </Router>
  );
}
