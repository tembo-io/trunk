import type { NextApiRequest, NextApiResponse } from "next";

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  try {
    const { ext } = req.query;
    await res.revalidate(`/extensions/${ext}`);
    await res.revalidate("/");
    return res.json({ revalidated: true });
  } catch (err) {
    return res.status(500).send("Error revalidating");
  }
}
