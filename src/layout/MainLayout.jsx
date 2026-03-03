import Sidebar from "../components/Sidebar"
import Editor from "../components/Editor"

export default function MainLayout() {
  return (
    <div className="app-container">
      <Sidebar/>
      <Editor />
    </div>
  )
}