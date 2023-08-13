import { BrowserRouter, Routes, Route } from "react-router-dom";
import {
  AppWrapper,
  MetaMaskConnectionPage,
  DashboardPage,
  CreateProjectPage,
  ContributeToProjectPage,
  ManageProjectPage,
  ReviewSubmissionPage,
  PageNotFound,
} from "./components";

function App() {
  return (
    <BrowserRouter>
      <AppWrapper>
        <Routes>
          <Route path="/" element={<MetaMaskConnectionPage />} />
          <Route path="/dashboard" element={<DashboardPage />} />
          <Route path="/project/create" element={<CreateProjectPage />} />
          <Route
            path="/project/:projectId/contribute"
            element={<ContributeToProjectPage />}
          />
          <Route
            path="/project/:projectId/manage"
            element={<ManageProjectPage />}
          />
          <Route
            path="/project/:projectId/submission/:submissionId"
            element={<ReviewSubmissionPage />}
          />
          <Route path="*" element={<PageNotFound />} />
        </Routes>
      </AppWrapper>
    </BrowserRouter>
  );
}

export default App;
