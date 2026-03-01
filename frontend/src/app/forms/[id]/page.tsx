const dummyForm = {
  title: "Customer Experience Survey",
  description:
    "Help us understand how we're doing by answering a few quick questions.",
  questions: [
    "What is your name?",
    "How satisfied are you with our service?",
    "What could we improve for next time?",
  ],
};

export default async function FormDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  return (
    <div className="min-h-[calc(100vh-4rem)] bg-slate-50 px-4 py-12">
      <div className="mx-auto flex w-full max-w-3xl flex-col gap-6">
        <div className="rounded-2xl border border-slate-200 bg-white p-8 shadow-sm">
          <div className="space-y-2">
            <p className="text-xs font-semibold uppercase tracking-wide text-slate-500">
              Form #{id}
            </p>
            <h1 className="text-2xl font-semibold text-slate-900">
              {dummyForm.title}
            </h1>
            <p className="text-sm text-slate-600">{dummyForm.description}</p>
          </div>
          <div className="mt-6 space-y-4">
            <h2 className="text-sm font-semibold text-slate-900">
              Questions (read-only)
            </h2>
            <ul className="space-y-3">
              {dummyForm.questions.map((question) => (
                <li
                  key={question}
                  className="rounded-xl border border-slate-200 bg-slate-50 px-4 py-3 text-sm text-slate-700"
                >
                  {question}
                </li>
              ))}
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
