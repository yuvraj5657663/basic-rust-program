const { APP_ID, PRIVATE_KEY, INSTALLATION_ID, CLIENT_PAYLOAD, EVENT_TYPE } =
  process.env;
import { App } from "@octokit/app";
(async () => {
  try {
    console.log("Triggering event");
    if (
      !APP_ID ||
      !PRIVATE_KEY ||
      !INSTALLATION_ID ||
      !CLIENT_PAYLOAD ||
      !EVENT_TYPE
    ) {
      console.error("Missing required environment variables");
      process.exit(1);
    } else {
      console.log("All required environment variables are present");
    }
    // Initialize the Octokit client with the App credentials
    const app = new App({
      appId: parseInt(`${APP_ID}`),
      privateKey: PRIVATE_KEY.replace(/\\n/g, "\n"),
    });
    const octokit = await app.getInstallationOctokit(
      parseInt(`${INSTALLATION_ID}`)
    );
    const payload = JSON.parse(CLIENT_PAYLOAD);
    await octokit.request(
      "POST /repos/{owner}/{repo}/dispatches",
      {
        owner: "StaytunedLLP",
        repo: "github-automation",
        event_type: EVENT_TYPE,
        client_payload: payload,
        headers: {
          "X-GitHub-Api-Version": "2022-11-28",
        },
      }
    );
    console.log("Event triggered successfully");
  } catch (error) {
    console.error("Error occurred while triggering event", error);
    process.exit(1);
  }

})();
