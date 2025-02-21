"use client";

import Image from "next/image";
import { useUser } from "@clerk/clerk-react";
import { Button } from "@/components/ui/button";
import { PlusCircleIcon } from "lucide-react";

const DocumentsPage = () => {
  const { user } = useUser();

  return (
    <div className="h-full flex flex-col items-center justify-center space-y-4">
      <Image
        src="/app/docs/empty.svg"
        height={300}
        width={300}
        alt="Empty"
        className="dark:"
      />
      <h2 className="text-lg font-medium">
        Welcome {user?.firstName}&apos;s Feytra
      </h2>
      <Button>
        <PlusCircleIcon className="h-4 w-4 mr-2" />
        Create a note
      </Button>
    </div>
  );
};

export default DocumentsPage;
