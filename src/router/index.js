import { createRouter, createWebHistory } from 'vue-router'
import RegFormView from '@/pages/RegForm.vue'
import MainVue from '@/pages/Main.vue'
import UserPage from "@/pages/UserPage.vue"
import Requests from "@/pages/Requests.vue"
import Edit from "@/pages/Edit.vue"

const routes = [
  {
    path: '/register',
    name: 'register',
    component: RegFormView
  },
  {
    path: '/',
    name: 'main',
    component: MainVue
  },
  {
    path: '/user/:searchAddress',
    name: 'user',
    component: UserPage
  },
  {
    path: '/requests',
    name: 'requests',
    component: Requests
  },
  {
    path: '/edit',
    name: 'edit',
    component: Edit
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// router.beforeEach((to, from, next) => {
//   if (to.path === '/') {
//     next('/register')
//   } else {
//     next()
//   }
// })

export default router
