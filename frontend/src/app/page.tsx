"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import Button from "@/components/ui/Button";

const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8000";

type FormSummary = {
  id: number | string;
  title: string;
  description: string;
};

export default function HomePage() {
  const [forms, setForms] = useState<FormSummary[]>([]);

  const loadForms = async () => {
    const response = await fetch(`${API_BASE_URL}/api/forms`);
    if (!response.ok) {
      return;
    }
    const data = (await response.json()) as FormSummary[];
    setForms(Array.isArray(data) ? data : []);
  };

  useEffect(() => {
    void loadForms();
  }, []);

  const handleCreateForm = async () => {
    const token = localStorage.getItem("token");
    const response = await fetch(`${API_BASE_URL}/api/forms`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({ title: "New Form", description: "Desc" }),
    });

    if (response.ok) {
      await loadForms();
    }
  };

  return (
    <div className="min-h-[calc(100vh-4rem)] bg-slate-50 px-4 py-12">
      <div className="mx-auto flex w-full max-w-6xl flex-col gap-8">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
          <div className="space-y-2">
            <h1 className="text-3xl font-semibold text-slate-900">
              Your forms
            </h1>
            <p className="text-sm text-slate-600">
              Manage and review every form you have created.
            </p>
          </div>
          <Button type="button" onClick={handleCreateForm}>
            Create new form
          </Button>
        </div>
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {forms.map((form) => (
            <Link
              key={form.id}
              href={`/forms/${form.id}`}
              className="rounded-2xl border border-slate-200 bg-white p-6 shadow-sm transition-shadow hover:shadow-md"
            >
              <div className="flex h-full flex-col justify-between gap-4">
                <div className="space-y-2">
                  <h2 className="text-lg font-semibold text-slate-900">
                    {form.title}
                  </h2>
                  <p className="text-sm text-slate-600">{form.description}</p>
                </div>
                <span className="text-sm font-medium text-slate-900">
                  View details →
                </span>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}
