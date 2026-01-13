export type Mail = {
  id: string;
  name: string;
  email: string;
  subject: string;
  text: string;
  date: string;
  read: boolean;
  labels: string[];
};

export const mails: Mail[] = [
  {
    id: "1",
    name: "William Smith",
    email: "williamsmith@example.com",
    subject: "Meeting Tomorrow",
    text: "Hi, let's have a meeting tomorrow to discuss the new project. I've attached the agenda for your review.",
    date: "2023-10-22T09:00:00",
    read: true,
    labels: ["meeting", "work"],
  },
  {
    id: "2",
    name: "Alice Johnson",
    email: "alicejohnson@example.com",
    subject: "Re: Project Update",
    text: "Thanks for the update. I'll take a look at the documents and get back to you by the end of the week.",
    date: "2023-10-22T10:30:00",
    read: false,
    labels: ["work"],
  },
  {
    id: "3",
    name: "Bob Wilson",
    email: "bobwilson@example.com",
    subject: "Weekend Plans",
    text: "Hey, are you free this weekend for a hike? The weather is supposed to be great!",
    date: "2023-10-21T15:45:00",
    read: true,
    labels: ["personal"],
  },
  {
    id: "4",
    name: "Emily Davis",
    email: "emilydavis@example.com",
    subject: "Invoicing Question",
    text: "I have a question regarding the last invoice. Can we jump on a quick call?",
    date: "2023-10-20T11:15:00",
    read: false,
    labels: ["work", "important"],
  },
  {
    id: "5",
    name: "Michael Brown",
    email: "michaelbrown@example.com",
    subject: "New Recipe",
    text: "You have to try this new recipe I found! It's amazing.",
    date: "2023-10-19T18:20:00",
    read: true,
    labels: ["personal"],
  },
  {
    id: "6",
    name: "Sarah Miller",
    email: "sarahmiller@example.com",
    subject: "Vacation Photos",
    text: "Here are the photos from our trip last month. It was so much fun!",
    date: "2023-10-18T14:10:00",
    read: true,
    labels: ["personal"],
  },
  {
    id: "7",
    name: "David Lee",
    email: "davidlee@example.com",
    subject: "Job Opportunity",
    text: "I saw a job opening that you might be interested in. Let me know if you want more details.",
    date: "2023-10-17T10:00:00",
    read: false,
    labels: ["work"],
  },
  {
    id: "8",
    name: "James Wilson",
    email: "jameswilson@example.com",
    subject: "Feedback Request",
    text: "Please provide your feedback on the latest draft of the proposal. Your input is valuable.",
    date: "2023-10-16T16:30:00",
    read: true,
    labels: ["work"],
  },
];
