import json
import tempfile
from os import path
from multiprocessing import Process

from firebase_admin import firestore, initialize_app  # type: ignore
from firebase_functions import scheduler_fn
import lolstaticdata.championrates.__main__  # type: ignore
import lolstaticdata.champions.__main__  # type: ignore
import lolstaticdata.items.__main__  # type: ignore


app = initialize_app()
db = firestore.client()


@scheduler_fn.on_schedule(schedule="every day 00:00")
def update_data(_event: scheduler_fn.ScheduledEvent):
    """Update database with the most recent champion and item data."""

    sets = ("championrates", "champions", "items")
    processes = [Process(target=_update_set, args=(s)) for s in sets]

    # Start then join each process.
    for f in (Process.start, Process.join):
        for p in processes:
            f(p)


def _update_set(name: str):
    """Update the given dataset: championrates, champions, or items."""

    # This is what we call BAD PRACTICES. I should probably submit a PR to
    # make lolstaticdata more friendly to use as a library.
    module = getattr(lolstaticdata, name)

    with tempfile.TemporaryDirectory() as temp:
        module.__main__.__file__ = path.join(temp, "a/b/c")
        module.__main__.main()

        with open(path.join(temp, f"{name}.json"), encoding="utf-8") as file:
            data = json.load(file)

            if name == "championrates":
                name = "champion-rates"
                metadata = {
                    "patch": data["patch"],
                    "timestamp": firestore.firestore.SERVER_TIMESTAMP,
                }
                db.collection("metadata").document("latest").set(metadata)

            for item in data.values():
                db.collection(name).document(item.id).set(item)
