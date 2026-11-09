import { CanMatchFn, RedirectCommand, Router, Routes } from '@angular/router';
import { NoTaskComponent } from './tasks/no-task/no-task.component';
import {
  resolveTitle,
  resolveUserName,
  UserTasksComponent,
} from './users/user-tasks/user-tasks.component';
import { TasksComponent } from './tasks/tasks.component';
import {
  canLeaveEditPage,
  NewTaskComponent,
} from './tasks/new-task/new-task.component';
import { NotFoundComponent } from './not-found/not-found.component';
import { inject } from '@angular/core';

const dummyCanMatch: CanMatchFn = (route, segments) => {
  const router = inject(Router);
  const shouldgetAcess = Math.random();
  if (shouldgetAcess < 0.5) {
    return true;
  }
  return new RedirectCommand(router.parseUrl('/unauthorized'));
};

export const routes: Routes = [
  {
    path: '', // your-doming/
    component: NoTaskComponent,
    // redirectTo: '/users/u1',
    // pathMatch: 'full',
    title: 'No task selected',
  },
  {
    path: 'users/:userId', // your-domain/users/<uid>
    component: UserTasksComponent,
    children: [
      {
        path: '',
        redirectTo: 'tasks',
        pathMatch: 'prefix',
      },
      {
        path: 'tasks', // your-domain/users/<uid>/tasks
        component: TasksComponent,
      },
      {
        path: 'tasks/new',
        component: NewTaskComponent,
      },
    ],
    // canMatch: [dummyCanMatch],
    data: {
      message: 'Hello!',
    },
    // runGuardsAndResolvers: 'paramsOrQueryParams',
    resolve: {
      userName: resolveUserName,
    },
    title: resolveTitle,
  },
  {
    path: 'tasks/new',
    component: NewTaskComponent,
    canDeactivate: [canLeaveEditPage],
  },
  {
    path: '**',
    component: NotFoundComponent,
  },
];
