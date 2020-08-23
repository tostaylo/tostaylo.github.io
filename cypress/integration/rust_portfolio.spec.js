describe('It Renders ', () => {
	it('Has default dark theme', () => {
		cy.visit('http://localhost:8000/');
		cy.get('#main').should('have.class', 'dark');
	});

	it('Render site onfo', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=nav-site-info]').click();
		cy.get('[data-cy=site-info]').should('be.visible');
		cy.get('[data-cy=psi-main-text]').should('be.visible');
	});

	it('Should switch themes', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=theme-toggle]').click();
		cy.get('[data-cy="Light Mode"]').click();
		cy.get('#main').should('have.class', 'light');
	});

	it('Should have a working mobile menu', () => {
		cy.visit('http://localhost:8000/');
		cy.viewport(320, 480);
		cy.get('[data-cy=nav-posts]').click();
		cy.get('[data-cy=menu-button]').click();
		cy.get('[data-cy=nav-about]').click();
		cy.get('[data-cy=about]').should('be.visible');
	});
});
