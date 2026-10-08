import {
  listProjects,
  createProject,
  deleteProject,
  getBible,
  saveBible,
  listDocuments,
  createDocument,
  type Project,
  type ProjectBible,
  type Document,
} from "$lib/api";

class ProjectStore {
  projects = $state<Project[]>([]);
  activeProject = $state<Project | null>(null);
  activeBible = $state<ProjectBible | null>(null);
  documents = $state<Document[]>([]);
  activeDoc = $state<Document | null>(null);
  loading = $state(false);

  async init() {
    await this.loadProjects();
  }

  async loadProjects() {
    this.loading = true;
    try {
      const list = await listProjects();
      this.projects = list;
      if (this.projects.length > 0 && !this.activeProject) {
        await this.selectProject(this.projects[0]);
      } else if (this.projects.length === 0) {
        const starter = await createProject({
          title: "Sovereign Engineering Studio",
          description: "Studio produksi konten teknis, e-book, dan peluncuran produk digital.",
          target_audience: "Software engineers, tech founders, and developers",
          default_tone: "Direct, Sovereign, Minimalist",
        });
        this.projects = [starter];
        await this.selectProject(starter);
      }
    } catch (e) {
      console.error("Gagal load projects:", e);
    } finally {
      this.loading = false;
    }
  }

  async selectProject(p: Project) {
    this.activeProject = p;
    try {
      this.activeBible = await getBible(p.id);
      await this.refreshDocuments();
    } catch (e) {
      console.error("Gagal select project:", e);
    }
  }

  async refreshDocuments() {
    if (!this.activeProject) return;
    try {
      this.documents = await listDocuments(this.activeProject.id);
      if (this.documents.length > 0 && !this.activeDoc) {
        this.activeDoc = this.documents[0];
      }
    } catch (e) {
      console.error("Gagal refresh documents:", e);
    }
  }

  async createNewDocument(title: string, docType = "custom") {
    if (!this.activeProject) return;
    const doc = await createDocument({
      project_id: this.activeProject.id,
      title,
      doc_type: docType,
      initial_content: `# ${title}\n\nMulai tulis naskah di sini...`,
    });
    this.documents.unshift(doc);
    this.activeDoc = doc;
    return doc;
  }
}

export const projectStore = new ProjectStore();
