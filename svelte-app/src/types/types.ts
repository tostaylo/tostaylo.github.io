export interface PostSummary {
	title: string;
	id: number;
	user: string;
	time_ago: string;
}

export interface PostType extends PostSummary {
	html_content: string;
}
